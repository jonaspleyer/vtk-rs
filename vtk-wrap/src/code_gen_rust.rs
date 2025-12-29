use std::ops::Deref;

use quote::{ToTokens, TokenStreamExt};

use crate::intermediate_representation::IRIdent;

// use crate::parse_cpp::StdFunction;

impl ToTokens for crate::IRType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use crate::IRType::*;
        use quote::quote;
        match self {
            Const(_) => (),
            _ => tokens.extend(quote!(mut)),
        }

        let ty = match self {
            Unit => quote::quote!(()),
            c_char => quote!(core::ffi::c_char),
            c_short => quote!(core::ffi::c_short),
            c_int => quote!(core::ffi::c_int),
            c_long => quote!(core::ffi::c_long),
            c_longlong => quote!(core::ffi::c_uchar),
            c_uchar => quote!(core::ffi::c_uchar),
            c_ushort => quote!(core::ffi::c_ushort),
            c_uint => quote!(core::ffi::c_uint),
            c_ulong => quote!(core::ffi::c_ulong),
            c_ulonglong => quote!(core::ffi::c_ulonglong),
            bool => quote!(bool),
            float => quote!(core::ffi::c_float),
            double => quote!(core::ffi::c_double),
            usize => quote!(usize),
            File => quote!(std::fs::File),
            FileMode => todo!(),
            Ref(irtype) => match irtype.as_ref() {
                Const(ir_ty) => quote!(&#ir_ty),
                a => quote!(&mut #a),
            },
            Pointer(irtype) => match irtype.as_ref() {
                Const(ir_ty) => quote!(*const #ir_ty),
                _ => quote!(*mut #irtype),
            },
            // This was handled above so we can simply apply the function to the inner type
            Const(irtype) => quote!(#irtype),
            String => quote!(String),
            Array(irtype, size) => quote!([#irtype; #size]),
            Vec(irtype) => quote!(Vec<#irtype>),
            Map(irtype1, irtype2) => quote!(std::collections::HashMap<#irtype1, #irtype2>),
            LinkedList(irtype) => quote!(std::collections::LinkedList<#irtype>),
            Path(p) => {
                let mut out = quote!();
                let n_max = p.0.len();
                for (i, path_segment) in p.0.iter().enumerate() {
                    let ident = quote::format_ident!("{}", path_segment);
                    out.extend(quote!(#ident));
                    if i + 1 < n_max {
                        out.extend(quote!(::));
                    }
                }
                out
            }
        };
        tokens.extend(ty);
    }
}

impl ToTokens for IRIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = quote::format_ident!("{}", self.ident);
        tokens.extend(quote::quote!(#id))
    }
}

impl ToTokens for crate::IRMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate::IRMethod {
            name,
            return_type,
            args,
        } = &self;

        let name = quote::format_ident!("{name}");
        let args = args.iter().map(|(name, ty)| quote::quote!(#name: #ty));
        let code = quote::quote!(fn #name (#(#args),*) -> #return_type;);

        tokens.extend(code);
    }
}

impl quote::ToTokens for crate::IRModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let modname = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut output = quote::quote!();
        for class in self.classes.values().take(2) {
            let mut methods = quote::quote!();
            for method in class.exposable_methods.iter().take(2) {
                methods.extend(quote::quote!(#method));
            }
            let class_name = syn::Ident::new(&class.name, proc_macro2::Span::call_site());
            output.extend(quote::quote!(
                impl #class_name {
                    #methods
                }
            ));
        }
        tokens.extend(quote::quote!(
            #[allow(non_camel_case_types)]
            pub mod #modname {
                #output
            }
        ));
    }
}

impl quote::ToTokens for crate::parse_cpp::Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let r = quote::format_ident!("{}", self.0);
        tokens.extend(quote::quote!(#r));
    }
}

impl quote::ToTokens for crate::parse_cpp::Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let p = self.0.iter().map(|s| quote::format_ident!("{s}"));
        let r = quote::quote!(#(#p)::*);
        tokens.extend(r);
    }
}
