use std::path::Path;

use eyre::Result;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::quote;
use syn::Ident;
use xshell::{cmd, read_file, write_file};

use crate::{token_def::TokenDef, utils};

impl TokenDef {
    fn gen_defs(&self) -> TokenStream {
        let imports = TokenDef::imports();

        let keyword_idents = self.get_keyword_idents();
        let keyword_tt = self.gen_keyword_tt();
        let keyword_terminals = gen_terminals(&keyword_idents);

        let literal_idents = self.get_literal_idents();
        let literal_tt = self.gen_literal_tt();
        let literal_terminals = gen_terminals(&literal_idents);

        let punct_idents = self.get_punct_idents();
        let punct_tt = self.gen_punct_tt();
        let punct_terminals = gen_terminals(&punct_idents);

        let token_stream = quote! {
            #imports

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum TokenKind {
                #(#keyword_idents,)*
                #(#literal_idents,)*
                #(#punct_idents,)*
                Ident,
                Eof,
            }

            #[macro_export]
            macro_rules! Tok {
                #([#punct_tt] => { $crate::ast::TokenKind::#punct_idents };)*
                #([#keyword_tt] => { $crate::ast::TokenKind::#keyword_idents};)*
                #([#literal_tt] => { $crate::ast::TokenKind::#literal_idents};)*
                [ident] => { $crate::ast::TokenKind::Ident };
                [eof] => { $crate::ast::TokenKind::Eof };
            }

            #(#literal_terminals)*
            #(#punct_terminals)*
            #(#keyword_terminals)*
        };

        token_stream
    }

    fn imports() -> TokenStream {
        quote! {
            use crate::parsing;
            use super::Token;
        }
    }

    fn get_keyword_idents(&self) -> Vec<Ident> {
        self.keywords
            .iter()
            .map(|kw| {
                let kw = kw.to_camel_case();
                format_ident!("Kw{}", kw)
            })
            .collect()
    }

    fn gen_keyword_tt(&self) -> Vec<TokenStream> {
        self.keywords
            .iter()
            .map(|kw| {
                let kw_ident = format_ident!("{}", kw);
                quote! { #kw_ident }
            })
            .collect()
    }

    fn get_literal_idents(&self) -> Vec<Ident> {
        self.literals
            .iter()
            .map(|lit| format_ident!("{}", lit))
            .collect()
    }

    fn gen_literal_tt(&self) -> Vec<TokenStream> {
        self.literals
            .iter()
            .map(|lit| {
                let lit = lit.to_snake_case();
                let lit_ident = format_ident!("{}", lit);
                quote! { #lit_ident }
            })
            .collect()
    }

    fn get_punct_idents(&self) -> Vec<Ident> {
        self.punct
            .keys()
            .map(|variant| format_ident!("{}", variant))
            .collect()
    }

    fn gen_punct_tt(&self) -> Vec<TokenStream> {
        self.punct
            .values()
            .map(|text| {
                if "{}[]()".contains(text) {
                    let c = text.chars().next().unwrap();
                    quote! { #c }
                } else {
                    let cs = text.chars().map(|c| Punct::new(c, Spacing::Joint));
                    quote! { #(#cs)* }
                }
            })
            .collect()
    }
}

fn gen_terminals(idents: &[Ident]) -> Vec<TokenStream> {
    idents
        .iter()
        .map(|variant| {
            quote! {
                pub struct #variant {
                    pub token: Token,
                }

                impl parsing::Parse for #variant {
                    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
                        let token = p.next()?;

                        match token.kind {
                            TokenKind::#variant => Ok(Self { token }),
                            _ => Err(parsing::ParseError::expected(&token, "abstract")),
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn run() -> Result<()> {
    let tokendef = TokenDef::get()?;
    let s = reformat(&tokendef.gen_defs().to_string())?;
    let path = utils::project_root().join("crates/monkey-lang/src/ast/generated.rs");
    update(&path, &s)?;
    Ok(())
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str) -> Result<()> {
    fn normalize(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    match read_file(path) {
        Ok(old_contents) if normalize(&old_contents) == normalize(contents) => {
            return Ok(());
        }
        _ => (),
    }

    eprintln!("updating {}", path.display());
    write_file(path, contents)?;
    Ok(())
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/src/codegen`";

pub fn reformat(text: &str) -> Result<String> {
    let stdout = cmd!("rustfmt").stdin(text).read()?;
    Ok(format!("//! {}\n\n{}\n", PREAMBLE, stdout))
}
