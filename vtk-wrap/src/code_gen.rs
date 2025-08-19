use std::ops::Deref;

use quote::ToTokens;

use crate::parse_cpp::StdFunction;

impl ToTokens for crate::parse_cpp::CppType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use crate::parse_cpp::CppType::*;
        let ty = match self {
            Void => quote::quote!(()),
            SignedChar => quote::quote!(),
            UnsignedChar => quote::quote!(u8),
            ShortInt => quote::quote!(i16),
            UnsignedShortInt => quote::quote!(u16),
            Int => quote::quote!(i32),
            UnsignedInt => quote::quote!(u32),
            LongInt => quote::quote!(i64),
            UnsignedLongInt => quote::quote!(u64),
            LongLongInt => quote::quote!(i128),
            UnsignedLongLongInt => quote::quote!(),
            LongDouble => quote::quote!(c_longdouble),
            Double => quote::quote!(f64),
            Float => quote::quote!(f32),
            Bool => quote::quote!(bool),
            Ostream => quote::quote!(),
            SizeT => quote::quote!(usize),
            File => quote::quote!(std::fs::File),
            FileMode => quote::quote!(),
            Function(func) => {
                let StdFunction { return_type, args } = func.as_ref();
                quote::quote!(fn(#(#args),*) -> #return_type)
            }
            // Containers
            Ref(ty) => {
                if let Const(_) = ty.deref() {
                    quote::quote!(& #ty)
                } else {
                    quote::quote!(&mut #ty)
                }
            }
            Pointer(ty) => {
                if let Const(_) = ty.deref() {
                    quote::quote!(*const #ty)
                } else {
                    quote::quote!(*mut #ty)
                }
            }
            Const(_) => quote::quote!(),
            // Standard Library
            String => quote::quote!(String),
            // TODO this is wrong!
            TypeInfo => quote::quote!(std::any::TypeId),
            Array(ty, n) => quote::quote!([#ty; #n]),
            Vec(ty) => quote::quote!(Vec<#ty>),
            Map(key, val) => quote::quote!(std::collections::HashMap<#key, #val>),
            LinkedList(ty) => quote::quote!(std::collections::LinkedList<#ty>),
            // Others
            Generic { pre, args } => {
                quote::quote!(#pre < #(#args),* >)
            }
            Path(path) => quote::quote!(#path),
        };
        tokens.extend(ty);
    }
}

impl ToTokens for crate::RustMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate::RustMethod {
            name,
            return_type,
            args,
        } = &self;

        let name = quote::format_ident!("{name}");
        let args = args.iter().map(|(name, ty)| quote::quote!(#name: #ty));

        let code = quote::quote!(
            fn #name (#(#args),*) -> #return_type {
                todo!()
            }
        );

        tokens.extend(code);
    }
}

impl quote::ToTokens for crate::RustModule {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        for class in self.classes.values() {
            for method in &class.exposable_methods {
                println!("{}", quote::quote!(#method));
            }
        }
        todo!()
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
