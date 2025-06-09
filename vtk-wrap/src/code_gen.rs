use crate::parsing::*;
use anyhow::{Context, Result};
use log::warn;
use std::str::FromStr;

macro_rules! ident(
    (@snake $($to:tt)*) => {
        proc_macro2::Ident::new_raw(
            &convert_case::Casing::to_case(&format!($($to)*), convert_case::Case::Snake),
            proc_macro2::Span::call_site()
        )
    };
    ($($to:tt)*) => {
        proc_macro2::Ident::new_raw(&format!($($to)*), proc_macro2::Span::call_site())
    };
);

pub struct Generator {
    pub hierarchy: crate::inheritance_hierarchy::Hierarchy,
}

impl Generator {
    pub fn new(modules: &[crate::parsing::Module]) -> Result<Self> {
        let hierarchy = crate::inheritance_hierarchy::Hierarchy::new(modules)?;

        Ok(Self { hierarchy })
    }

    pub fn format_code(tokenstream: proc_macro2::TokenStream) -> Result<String> {
        let file: Result<syn::File, _> = syn::parse_file(&tokenstream.to_string());
        match file {
            Ok(res) => Ok(prettyplease::unparse(&res)),
            Err(e) => {
                warn!("Formatting failed: \"{e}\" Returning unformatted Code");
                Ok(tokenstream.to_string())
            }
        }
    }

    pub fn generate_trait(&self, class: &Class) -> Result<Option<String>> {
        if !self.hierarchy.has_dependant(class) {
            return Ok(None);
        }

        let methods = self
            .hierarchy
            .get_non_inherited_public_methods(&class.name)?
            .into_iter()
            .map(|method| gen_method(class, &method).map(|x| x.1))
            .collect::<Result<Vec<_>>>()?;

        let name = proc_macro2::TokenStream::from_str(&class.name)
            .ok()
            .context("Cannot parse class name")?;

        if methods.is_empty() {
            Ok(None)
        } else {
            let code = quote::quote!(
                pub trait #name {
                    #(#methods)*
                }
            );
            Ok(Some(Self::format_code(code)?))
        }
    }
}

pub fn translate_type(ttype: &str) -> proc_macro2::TokenStream {
    match ttype {
        "const char" | "char" => quote::quote!(char),
        "const char*" | "char*" | "std::string" | "const std::string" => quote::quote!(String),
        "const double" | "double" => quote::quote!(f64),
        "const float" | "float" => quote::quote!(f32),
        "const int" | "int" | "const long" | "long" => quote::quote!(i64),
        "const longlong" | "longlong" => quote::quote!(i64),
        "const bool" | "bool" => quote::quote!(bool),
        "void" => quote::quote!(()),
        "const unsigned int" | "unsigned int" | "const unsigned long" | "unsigned long" => {
            quote::quote!(u64)
        }
        other => {
            let mut ttype = other.trim().replace(" ", "_");

            if ttype.contains("*") {
                ttype = format!("&{}", ttype.replace("*", ""));
            // Detect if we have a type which represents a pointer
            } else if ttype.contains("**") {
                ttype = format!("&[{}]", ttype.replace("*", ""));
            }
            proc_macro2::TokenStream::from_str(&ttype).unwrap()
        }
    }
}

pub fn convert_ident(n: usize, ident: &Option<&String>) -> Result<proc_macro2::TokenStream> {
    if let Some(ident) = ident {
        if *ident == "self" {
            Ok(quote::quote!(sself))
        } else {
            let ident = ident!("{}", ident);
            Ok(quote::quote!(#ident))
        }
    } else {
        proc_macro2::TokenStream::from_str(&format!("value{n}"))
            .ok()
            .context("Could not parse string to tokenstream")
    }
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
    if access == &Access::Private {
        return Ok((quote::quote!(), quote::quote!()));
    }

    // Determine &self, &mut self or nothing (for static methods)
    let (mut args_all, mut args_method) = if *is_static {
        (vec![], vec![])
    } else if *is_const {
        (
            vec![quote::quote!(&self)],
            vec![quote::quote!(self.as_pointer())],
        )
    } else {
        (
            vec![quote::quote!(&mut self)],
            vec![quote::quote!(self.as_pointer_mut())],
        )
    };

    let _args = parameters
        .iter()
        .enumerate()
        .map(|(n, param)| {
            let name = convert_ident(n, &param.name.as_ref())?;
            let ttype = translate_type(&param.r#type);
            Ok((name, ttype))
        })
        .collect::<Result<Vec<_>>>()
        .ok()
        .context("could not construct tokenstream")?;

    args_all.extend(_args.iter().map(|(n, t)| quote::quote!(#n: #t)));
    args_method.extend(_args.into_iter().map(|(name, _)| name));

    // let name = proc_macro2::Ident::new(name.to_case(Case::Snake), proc_macro2::Span::call_site());
    let name_ffi = ident!(@snake "{}_{name}", class.name);
    let name_impl = ident!(@snake "{name}");
    let return_type = return_type
        .as_ref()
        .map(|x| translate_type(&x.ret_type))
        .unwrap_or(quote::quote!(()));

    let gen_ffi = quote::quote!(
        fn #name_ffi(#(#args_all),*) -> #return_type;
    );
    let gen_impl = quote::quote!(
        #[doc(alias = #name)]
        #[inline(always)]
        fn #name_impl(#(#args_all),*) -> #return_type {
            ffi::#name_ffi(#(#args_method),*)
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
        .public
        .iter()
        .filter(|method| !method.is_virtual)
        .filter_map(|method| match gen_method(class, method) {
            Ok(v) => Some(v),
            Err(e) => {
                errors.push(e);
                None
            }
        })
        .unzip();

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
