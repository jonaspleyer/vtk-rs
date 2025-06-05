use crate::parsing::*;
use anyhow::{Context, Result};

pub fn gen_wrapper(class: &Class) -> Result<proc_macro2::TokenStream> {
    let name = proc_macro2::Ident::new(&class.name, proc_macro2::Span::call_site());
    let fields = class.members.iter().map(|member| {
        let field_name = proc_macro2::Ident::new(&member.name, proc_macro2::Span::call_site());
        quote::quote!(#field_name: bool)
    });
    Ok(quote::quote!(struct #name {
        #(#fields),*
    }))
}
