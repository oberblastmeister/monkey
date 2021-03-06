use std::{ops::Deref, path::Path, slice, vec};

use eyre::Result;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::quote;
use syn::Ident;
use xshell::{cmd, read_file, write_file};

use crate::{
    token_def::{TokenDef, TokenDefs, TokenType},
    utils,
};

impl TokenDefs {
    fn lower(&self) -> TokenGens {
        TokenGens(self.tokens.iter().map(|v| v.lower()).collect::<Vec<_>>())
    }

    fn gen_defs(&self) -> TokenStream {
        let token_gens = self.lower();

        let keyword_token_gens =
            token_gens.iter().filter(|gen| gen.ttype == TokenType::Keyword).collect::<Vec<_>>();

        let tts = token_gens.tts();

        let docs = token_gens.docs();

        let variants = token_gens.variants();

        let terminals = token_gens.gen_terminals();

        let imports = TokenDefs::imports();

        let as_str_fn = token_gens.gen_as_str();

        let token_stream = quote! {
            #imports

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum TokenKind {
                #(
                    #docs
                    #variants,
                )*
                Eof,
                Error,
            }

            /// A helper macro to get the token kind
            #[macro_export]
            macro_rules! K {
                #([#tts] => { $crate::ast::TokenKind::#variants };)*
                // [ident] => { $crate::ast::TokenKind::Ident };
                [eof] => { $crate::ast::TokenKind::Eof };
            }

            #[macro_export]
            /// A helper macro to get the terminal type
            macro_rules! T {
                #([#tts] => { $crate::ast::generated::#variants };)*
                // [ident] => { $crate::ast::generated::Ident };
            }

            impl TokenKind {
                #as_str_fn
            }

            impl fmt::Display for TokenKind {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(self.as_str())
                }
            }

            #(#terminals)*
        };

        token_stream
    }

    fn imports() -> TokenStream {
        quote! {
            use crate::parsing;
            use super::Token;
            use std::fmt;
        }
    }
}

impl TokenDef {
    fn lower(&self) -> TokenGen {
        let doc = self.doc.clone().map(|s| quote! { #[doc = #s] });

        match self.ttype {
            TokenType::Keyword => {
                let text = self.text.clone().unwrap();
                let tt = format_ident!("{}", &text);
                let variant_s =
                    self.variant.clone().unwrap_or_else(|| format!("{}Kw", text.to_camel_case()));
                let variant = format_ident!("{}", variant_s);
                TokenGen { text: Some(text), tt: quote! { #tt }, variant, doc, ttype: self.ttype }
            }
            TokenType::Literal => {
                let tt = format_ident!("{}", self.variant.as_ref().unwrap().to_snake_case());
                let variant = format_ident!("{}", self.variant.as_ref().unwrap());
                TokenGen { text: None, tt: quote! { #tt }, variant, doc, ttype: self.ttype }
            }
            TokenType::Punct => {
                let text = self.text.clone().unwrap();
                let tt = if "{}[]()".contains(&text) {
                    let c = text.chars().next().unwrap();
                    quote! { #c }
                } else {
                    let cs = text.chars().map(|c| Punct::new(c, Spacing::Joint));
                    quote! { #(#cs)* }
                };
                let variant = format_ident!("{}", self.variant.as_ref().unwrap());
                TokenGen { text: Some(text), tt, variant, doc, ttype: self.ttype }
            }
            TokenType::Token => {
                if self.text.as_ref().is_some() {
                    panic!("This token tokendefs cannot have text: {:?}", self);
                }

                let tt = format_ident!("{}", self.variant.as_ref().unwrap().to_snake_case());
                let variant = format_ident!("{}", self.variant.as_ref().unwrap());

                TokenGen { text: None, tt: quote! { #tt }, variant, doc, ttype: self.ttype }
            }
        }
    }
}

struct TokenGens(Vec<TokenGen>);

impl TokenGens {
    fn iter(&self) -> slice::Iter<TokenGen> {
        self.0.iter()
    }

    fn tts(&self) -> Vec<&TokenStream> {
        self.iter().map(|gen| &gen.tt).collect()
    }

    fn docs(&self) -> Vec<Option<&TokenStream>> {
        self.iter().map(|gen| gen.doc.as_ref()).collect()
    }

    fn variants(&self) -> Vec<TokenStream> {
        self.iter()
            .map(|gen| {
                let variant = &gen.variant;
                quote! { #variant }
            })
            .collect()
    }

    fn gen_terminals(&self) -> Vec<TokenStream> {
        self.iter().map(|gen| gen.gen_terminal()).collect()
    }

    fn gen_as_str(&self) -> TokenStream {
        let arms = self.iter().map(|TokenGen { text, variant, .. }| {
            let text = text.clone().unwrap_or_else(|| variant.to_string().to_snake_case());

            quote! {
                Self::#variant => #text
            }
        });

        quote! {
            /// Get the display of the TokenKind
            pub fn as_str(&self) -> &'static str {
                match self {
                    #(#arms,)*
                    Self::Eof => "eof",
                    Self::Error => "error",
                }
            }
        }
    }
}

struct TokenGen {
    ttype: TokenType,
    text: Option<String>,
    tt: TokenStream,
    doc: Option<TokenStream>,
    variant: Ident,
}

impl TokenGen {
    fn gen_terminal(&self) -> TokenStream {
        let variant = &self.variant;

        let text = self.text.as_ref().map(|s| match s.deref() {
            "{" => "{{",
            "}" => "}}",
            _ => s,
        });

        let display_impl = text.map(|s| {
            quote! {
                impl fmt::Display for #variant {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, #s)
                    }
                }
            }
        });

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #variant {
                pub token: Token,
            }

            impl parsing::Parse for #variant {
                fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
                    let token = p.next()?;

                    match token.kind {
                        TokenKind::#variant => Ok(Self { token }),
                        _ => Err(parsing::ParseError::expected(&token, TokenKind::#variant.as_str())),
                    }
                }
            }

            impl parsing::Peek for #variant {
                fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
                    matches!(peeker.nth(0), TokenKind::#variant)
                }
            }

            #display_impl
        }
    }
}

pub fn run() -> Result<()> {
    let tokendef = TokenDefs::get()?;
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
