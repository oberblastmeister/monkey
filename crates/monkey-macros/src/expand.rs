use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataEnum, DeriveInput, Error, Fields, Ident, Meta, MetaList, NestedMeta, Result, Variant,
};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    match &node.data {
        Data::Enum(data_enum) => expand_enum(&node.ident, data_enum),
        _ => Err(Error::new_spanned(node, "Only enums are supported")),
    }
}

fn expand_enum(ident: &Ident, node: &DataEnum) -> Result<TokenStream> {
    let mut counter = 0;

    let arms = node
        .variants
        .iter()
        .map(|v| expand_variant(ident, v, &mut counter))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl #ident {
            fn precedence(&self) -> monkey_util::PrecedenceType {
                match self {
                    #(#arms,)*
                }
            }
        }
    })
}

fn expand_variant(ident: &Ident, node: &Variant, counter: &mut u8) -> Result<TokenStream> {
    match node.fields {
        Fields::Unit => (),
        _ => {
            return Err(Error::new_spanned(
                node,
                "Only unnamed fields are supported",
            ))
        }
    };

    if node.attrs.len() != 1 {
        return Err(Error::new_spanned(node, "Must only have one attribute"));
    }

    let attr = Attr::from_meta(node.attrs[0].parse_meta()?)?;
    let prec_type = attr.prec_type;
    let assoc = attr.assoc;

    let node_ident = &node.ident;

    let tokens = match prec_type {
        PrecType::Prefix => quote! {
            #ident::#node_ident => monkey_util::PrecedenceType::Prefix((), monkey_util::__precedence(#counter))
        },
        PrecType::Postfix => quote! {
            #ident::#node_ident => monkey_util::PrecedenceType::Postfix(monkey_util::__precedence(#counter), ())
        },
        PrecType::Infix => {
            let (first, second) = match assoc.unwrap() {
                Assoc::Left => {
                    let first = *counter;
                    *counter += 1;
                    (first, *counter)
                }
                Assoc::Right => {
                    let second = *counter;
                    *counter += 1;
                    (*counter, second)
                }
            };

            quote! {
                #ident::#node_ident => monkey_util::PrecedenceType::Infix(monkey_util::__precedence(#first), monkey_util::__precedence(#second))
            }
        }
    };

    *counter += 1;

    Ok(tokens)
}

#[derive(PartialEq)]
enum PrecType {
    Prefix,
    Postfix,
    Infix,
}

impl FromStr for PrecType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        Ok(match s {
            "prefix" => PrecType::Prefix,
            "postfix" => PrecType::Postfix,
            "infix" => PrecType::Infix,
            _ => return Err(()),
        })
    }
}

#[derive(PartialEq)]
enum Assoc {
    Left,
    Right,
}

impl FromStr for Assoc {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        Ok(match s {
            "left" => Assoc::Left,
            "right" => Assoc::Right,
            _ => return Err(()),
        })
    }
}

pub struct Attr {
    prec_type: PrecType,
    assoc: Option<Assoc>,
}

impl Attr {
    fn from_meta(meta: Meta) -> Result<Attr> {
        let meta_list = match meta {
            Meta::List(meta) => meta,
            _ => return Err(Error::new_spanned(meta, "Must a be list meta")),
        };

        let names = parse_meta_list(meta_list.clone())?;

        let prec_type = names[0].parse::<PrecType>().map_err(|_| {
            Error::new_spanned(
                &meta_list,
                "Second argument could not be parsed into a precedence type",
            )
        })?;

        let assoc = if prec_type == PrecType::Infix {
            let assoc = names[1].parse::<Assoc>().map_err(|_| {
                Error::new_spanned(&meta_list, "Must have association if it is infix")
            })?;
            Some(assoc)
        } else {
            None
        };

        Ok(Attr { assoc, prec_type })
    }
}

fn parse_meta_list(metas: MetaList) -> Result<Vec<String>> {
    metas
        .nested
        .into_iter()
        .map(|m| {
            let path = match m {
                NestedMeta::Meta(ref meta) => match meta {
                    Meta::Path(path) => path,
                    _ => {
                        return Err(Error::new_spanned(
                            m,
                            "Meta inside of nested meta must be a path",
                        ))
                    }
                },
                _ => return Err(Error::new_spanned(m, "Nested meta must be another meta")),
            };

            if path.segments.len() != 1 {
                return Err(Error::new_spanned(path, "Path must be one segment long"));
            }

            let last = path.segments.last().unwrap();
            let name = last.ident.to_string();

            Ok(name)
        })
        .collect()
}
