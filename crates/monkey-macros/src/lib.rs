mod expand;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};
use synstructure::{decl_derive, Structure};

#[proc_macro_derive(Precedent, attributes(prec))]
pub fn derive_precedent(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).unwrap_or_else(|err| err.to_compile_error()).into()
}

fn derive_spanned(s: Structure) -> TokenStream {
    let none_derived = quote! {
        There were no fields that could be derived on
    };

    let body = s.fold(quote! { #none_derived }, |acc, bi| {
        if acc.to_string() == quote! { #none_derived }.to_string() {
            quote! { #bi.span() }
        } else {
            quote! { #acc.cover(#bi.span()) }
        }
    });

    s.gen_impl(quote! {
        gen impl crate::spanned::Spanned for @Self {
            fn span(&self) -> crate::spanned::Span {
                match *self { #body }
            }
        }
    })
}
decl_derive!([Spanned] => derive_spanned);

fn check_not_unit_structure_or_enum(s: &Structure) {
    let variants = s.variants();

    assert!(variants.len() == 1, "Parse can only be implemented for structs!");
    assert!(
        variants[0].ast().fields != &Fields::Unit,
        "Parse cannot be implemented for structs that have unit fields"
    );
}

fn derive_parse(s: Structure) -> TokenStream {
    check_not_unit_structure_or_enum(&s);

    let body = s.variants()[0].construct(|_, _| {
        quote! { p.parse()? }
    });

    s.gen_impl(quote! {
        gen impl crate::Parse for @Self {
            fn parse(p: &mut crate::Parser) -> crate::ParseResult<Self> {
                Ok(#body)
            }
        }
    })
}
decl_derive!([Parse] => derive_parse);

fn derive_shrink_to_fit(s: Structure) -> TokenStream {
    check_not_unit_structure_or_enum(&s);

    let body = s.each(|bi| quote! {
        #bi.shrink_to_fit()
    });

    s.gen_impl(quote! {
        gen impl crate::traits::ShrinkToFit for @Self {
            fn shrink_to_fit(&mut self) {
                match self { #body }
            }
        }
    })
}
decl_derive!([ShrinkToFit] => derive_shrink_to_fit);
