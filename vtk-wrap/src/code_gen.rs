use crate::parsing::*;
use anyhow::{Context, Result};
use std::str::FromStr;

macro_rules! ident(
    (@snake $($to:tt)*) => {
        proc_macro2::Ident::new(
            &convert_case::Casing::to_case(&format!($($to)*), convert_case::Case::Snake),
            proc_macro2::Span::call_site()
        )
    };
    ($($to:tt)*) => {
        proc_macro2::Ident::new(&format!($($to)*), proc_macro2::Span::call_site())
    };
);

pub fn translate_type(r#type: &str) -> proc_macro2::TokenStream {
    let ttype = r#type.trim();
    match ttype {
        "const char" | "char" => quote::quote!(String),
        "double" => quote::quote!(f64),
        "float" => quote::quote!(f32),
        "int" | "long" => quote::quote!(i64),
        "longlong" => quote::quote!(i64),
        "bool" => quote::quote!(bool),
        "void" => quote::quote!(()),
        "unsigned int" | "unsigned long" => quote::quote!(u64),
        other => {
            let other_new = other.replace(" ", "");
            // quote::quote!(#other_new)
            proc_macro2::TokenStream::from_str(&other_new).unwrap()
        }
    }
}

pub fn convert_type(r#type: &str) -> String {
    r#type.replace(" ", "_")
}

pub fn convert_ident(n: usize, ident: &Option<&String>) -> String {
    ident
        .unwrap_or(&format!("value{n}"))
        .replace("type", "r#type")
}

pub fn gen_method(
    class: &Class,
    method: &Method,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let Method {
        name,
        property,
        access,
        is_const,
        is_static,
        is_virtual,
        signature,
        parameters,
        comment,
        return_type,
    } = &method;

    // We do not provide wrappers for private methods
    if access == &Some(Access::Private) {
        return Ok((quote::quote!(), quote::quote!()));
    }

    // Determine &self, &mut self or nothing (for static methods)
    let sself = if is_static.is_some_and(|x| x == 1) {
        quote::quote!()
    } else if is_const.is_some_and(|x| x == 1) {
        quote::quote!(&self,)
    } else {
        quote::quote!(&mut self,)
    };

    let args = parameters
        .iter()
        .enumerate()
        .map(|(n, param)| {
            let name = proc_macro2::TokenStream::from_str(&convert_ident(n, &param.name.as_ref()))?;
            let ttype = convert_type(&param.r#type);
            let ttype = proc_macro2::TokenStream::from_str(&ttype)?;
            Ok(quote::quote!(#name: #ttype))
        })
        .collect::<Result<Vec<_>, proc_macro2::LexError>>()
        .ok()
        .context("could not construct tokenstream")?;

    // let name = proc_macro2::Ident::new(name.to_case(Case::Snake), proc_macro2::Span::call_site());
    let name_ffi = ident!(@snake "{}_{name}", class.name);
    let name_impl = ident!(@snake "{name}");
    let return_type = return_type
        .as_ref()
        .map(|x| translate_type(&x.ret_type))
        .unwrap_or(quote::quote!(()));

    let gen_ffi = quote::quote!(
        fn #name_ffi(#(#args),*) -> #return_type;
    );
    let gen_impl = quote::quote!(
        fn #name_impl(#sself #(#args),*) -> #return_type {
            ffi::#name_ffi()
        }
    );

    Ok((gen_ffi, gen_impl))
}

pub fn gen_wrapper(class: &Class) -> Result<String> {
    let name = proc_macro2::Ident::new(&class.name, proc_macro2::Span::call_site());
    let fields = class.members.iter().map(|member| {
        let field_name = proc_macro2::Ident::new(&member.name, proc_macro2::Span::call_site());
        quote::quote!(#field_name: bool)
    });

    let mut errors = vec![];
    let (methods_ffi, methods_impl): (Vec<_>, Vec<_>) = class
        .methods
        .iter()
        .filter_map(|method| match gen_method(class, method) {
            Ok(v) => Some(v),
            Err(e) => {
                errors.push(e);
                None
            }
        })
        .unzip();

    // let methods_ffi = vec![&methods_ffi[0], &methods_ffi[1]];
    // let methods_impl = vec![&methods_impl[0], &methods_impl[1]];

    if let Some(e) = errors.into_iter().next() {
        return Err(e);
    }

    let output = quote::quote!(
        struct #name {
            #(#fields),*
        }

        mod ffi {
            unsafe extern "C" {
                #(#methods_ffi)*
            }
        }

        impl #name {
            #(#methods_impl)*
        }
    );

    let file: syn::File = syn::parse_file(&output.to_string())?;

    Ok(prettyplease::unparse(&file))
}
