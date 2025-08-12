use crate::parse_wrap_vtk_xml::*;
use anyhow::{Context, Result};
use log::warn;
use quote::ToTokens;
use std::str::FromStr;

trait TokenStreamToError<T> {
    fn convert(self) -> anyhow::Result<T>;
}

impl<T> TokenStreamToError<T> for Result<T, proc_macro2::LexError> {
    fn convert(self) -> anyhow::Result<T> {
        self.ok().context("Could not convert to TokenStream")
    }
}

macro_rules! ident(
    (@snake $($to:tt)*) => {
        proc_macro2::Ident::new_raw(
            &convert_case::Casing::to_case(&format!($($to)*), convert_case::Case::Snake),
            proc_macro2::Span::call_site()
        )
    };
    (@non_raw $($to:tt)*) => {
        proc_macro2::Ident::new(&format!($($to)*), proc_macro2::Span::call_site())
    };
    ($($to:tt)*) => {
        proc_macro2::Ident::new_raw(&format!($($to)*), proc_macro2::Span::call_site())
    };
);

pub struct Generator {
    pub hierarchy: crate::inheritance_hierarchy::ClassHierarchy,
}

impl Generator {
    pub fn new(modules: &[crate::parse_wrap_vtk_xml::Module]) -> Result<Self> {
        let hierarchy = crate::inheritance_hierarchy::ClassHierarchy::new(modules)?;

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

    fn translate_type(&self, ttype: &str) -> Result<proc_macro2::TokenStream> {
        let mut ttype = ttype.to_string();

        // Remove "const " which preceeds the type
        let mut modifier = None;
        if ttype.starts_with("const ") {
            ttype = ttype.chars().skip(6).collect();
            modifier = Some(Modifier::Const);
        }

        let ttype = if ttype.is_empty() || ttype == "void" {
            quote::quote!(())
        } else {
            // let p: proc_macro2::TokenStream = quote::quote!(#shared_generic_type)
            //     .parse()
            //     .ok()
            //     .context(format!("could not parse type {}", ttype))?;
            // p

            // Check if we have generic arguments
            let sgt = SharedGenericType::parse(&ttype)?.convert()?;
            quote::quote!(#sgt)
        };
        Ok(quote::quote!(#modifier #ttype))
    }

    pub fn gen_method(
        &self,
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
                let ttype = self.translate_type(&param.r#type)?;
                Ok((name, ttype))
            })
            .collect::<Result<Vec<_>>>()?;

        args_all.extend(_args.iter().map(|(n, t)| quote::quote!(#n: #t)));
        args_method.extend(_args.into_iter().map(|(name, _)| name));

        let method_doc = comment_to_docs(comment);

        // let name = proc_macro2::Ident::new(name.to_case(Case::Snake), proc_macro2::Span::call_site());
        let name_ffi = ident!(@snake "{}_{name}", class.name);
        let name_impl = ident!(@snake "{name}");
        let return_type = return_type
            .as_ref()
            .map(|x| self.translate_type(&x.ret_type))
            .unwrap_or(Ok(quote::quote!(())))?;

        let gen_ffi = quote::quote!(
            fn #name_ffi(#(#args_all),*) -> #return_type;
        );
        let gen_impl = quote::quote!(
            // #(#[doc = #method_doc])*
            // #[doc(alias = #name)]
            // #[inline(always)]
            fn #name_impl(#(#args_all),*) -> #return_type {
                ffi::#name_ffi(#(#args_method),*)
            }
        );

        Ok((gen_ffi, gen_impl))
    }

    pub fn generate_trait(&self, class: &Class) -> Result<Option<String>> {
        log::info!("Generating trait code for class {}", class.name);
        if !self.hierarchy.has_dependant(class) {
            return Ok(None);
        }

        let methods = self
            .hierarchy
            .get_non_inherited_public_methods(&class.name)?
            .into_iter()
            .map(|method| self.gen_method(class, &method).map(|x| x.1))
            .collect::<Result<Vec<_>>>()?;

        let name = proc_macro2::TokenStream::from_str(&class.name).convert()?;
        let class_doc = comment_to_docs(&class.comment.as_ref().map(|x| x.replace("@brief ", "")));

        if methods.is_empty() {
            Ok(None)
        } else {
            let code = quote::quote!(
                #(#[doc = #class_doc])*
                pub trait #name {
                    #(#methods)*
                }
            );
            Ok(Some(Self::format_code(code)?))
        }
    }

    pub fn gen_wrapper(&self, class: &Class) -> Result<String> {
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
            .filter_map(|method| match self.gen_method(class, method) {
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
}

pub enum SharedBuiltins {
    // Builtins
    Float32,
    Float64,
    Float128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Bool,
    String,
    Str,
    Char,
    // From C++ stdlib
    Array,
    Map,
    Vec,
    LinkedList,
    Other(String),
}

impl quote::ToTokens for SharedBuiltins {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use SharedBuiltins::*;
        let stream = match &self {
            Float32 => quote::quote!(f32),
            Float64 => quote::quote!(f64),
            Float128 => quote::quote!(f128),
            Int8 => quote::quote!(i8),
            Int16 => quote::quote!(i16),
            Int32 => quote::quote!(i32),
            Int64 => quote::quote!(i64),
            Int128 => quote::quote!(i128),
            UInt8 => quote::quote!(u8),
            UInt16 => quote::quote!(u16),
            UInt32 => quote::quote!(u32),
            UInt64 => quote::quote!(u64),
            UInt128 => quote::quote!(u128),
            Bool => quote::quote!(bool),
            String => quote::quote!(String),
            Str => quote::quote!(&str),
            Char => quote::quote!(char),
            // => quote::quote!() From C++ stdlib
            Array => quote::quote!([]),
            Map => quote::quote!(std::collections::HashMap),
            Vec => quote::quote!(Vec),
            LinkedList => quote::quote!(std::collections::LinkedList),
            Other(s) => {
                let stream: proc_macro2::TokenStream = s.parse().unwrap();
                // let id = ident!(@non_raw "{}", s);
                // quote::quote!(#id)
                stream
            }
        };
        tokens.extend(stream);
    }
}

impl SharedBuiltins {
    fn from_cpp_type(ttype: &str) -> Self {
        use SharedBuiltins::*;
        match ttype {
            // Builtins
            "signed char" => Int8,
            "short" | "short int" | "signed short" | "signed short int" => Int16,
            "int" | "signed" | "signed int" => Int32,
            "long" | "long int" | "signed long" | "signed long int" => Int64,
            "long long" | "long long int" | "signed long long" | "signed long long int" => Int128,
            "unsigned char" => UInt8,
            "unsigned short" | "unsigned short int" => UInt16,
            "unsigned" | "unsigned int" => UInt32,
            "unsigned long" | "unsigned long int" => UInt64,
            "unsigned long long" | "unsigned long long int" => UInt128,
            "double" => Float64,
            "long double" => Float128,
            "float" => Float32,
            "char" => Char,
            // From C++ stdlib
            "std::array" => Array,
            "std::map" => Map,
            "std::vector" => Vec,
            "std::list" => LinkedList,
            other => Other(other.to_string()),
        }
    }

    fn get_number_of_generic_args(&self) -> usize {
        use SharedBuiltins::*;
        match self {
            // Builtins
            Float32 => 0,
            Float64 => 0,
            Float128 => 0,
            Int8 => 0,
            Int16 => 0,
            Int32 => 0,
            Int64 => 0,
            Int128 => 0,
            UInt8 => 0,
            UInt16 => 0,
            UInt32 => 0,
            UInt64 => 0,
            UInt128 => 0,
            Bool => 0,
            String => 0,
            Str => 0,
            Char => 0,
            // From C++ stdlib
            Array => 2,
            Map => 2,
            Vec => 1,
            LinkedList => 1,
            Other(_) => 0,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
enum SharedGenericType<T> {
    Type(T),
    List {
        pre: T,
        args: Vec<SharedGenericType<T>>,
    },
}

pub enum Modifier {
    Ref,
    Const,
    ConstRef,
    RefToConst,
    Pointer,
    DoublePointer,
}

impl Modifier {
    fn wrap_type(&self, ty: &impl quote::ToTokens) -> proc_macro2::TokenStream {
        match self {
            Modifier::ConstRef | Modifier::RefToConst => quote::quote!(& #ty),
            Modifier::Pointer | Modifier::Ref => quote::quote!(&mut #ty),
            Modifier::Const => quote::quote!(#ty),
            Modifier::DoublePointer => quote::quote!(&[&mut #ty]),
        }
    }
}

impl quote::ToTokens for Modifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use Modifier::*;
        let m = match self {
            Ref => quote::quote!(&mut ),
            Const => quote::quote!(),
            ConstRef => quote::quote!(&),
            RefToConst => quote::quote!(&),
            Pointer => quote::quote!(&),
            DoublePointer => todo!(),
        };
        tokens.extend(m);
    }
}

pub struct SharedType {
    ty: SharedGenericType<SharedBuiltins>,
    modifier: Option<Modifier>,
}

impl quote::ToTokens for SharedType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let modifier = if let Some(modifier) = &self.modifier {
            use Modifier::*;
            match modifier {
                Ref => quote::quote!(mut &mut),
                Const => quote::quote!(),
                ConstRef => quote::quote!(&mut),
                RefToConst => quote::quote!(mut &),
                Pointer => quote::quote!(*mut ),
                DoublePointer => quote::quote!(*mut *mut),
            }
        } else {
            quote::quote!()
        };
        use SharedGenericType::*;
        let ty = match &self.ty {
            List { pre, args } => todo!(),
            Type(t) => quote::quote!(#t),
        };
        let stream = quote::quote!(#modifier #ty);
        tokens.extend(stream);
    }
}

impl SharedType {
    fn from_cpp(input: &str) -> Result<Self> {
        Ok(SharedType {
            ty: SharedGenericType::Type(SharedBuiltins::from_cpp_type(input)),
            modifier: None,
        })
    }
}

impl SharedGenericType<String> {
    fn parse(input: &str) -> Result<Self> {
        if let Some((pre, tail)) = input.split_once("<") {
            let tail = tail.trim();
            anyhow::ensure!(tail.ends_with(">"));
            let mut args: Vec<Self> = vec![];

            let mut record = String::new();
            let mut nesting = 1isize;
            for char in tail.chars() {
                if char == '<' {
                    nesting += 1
                } else if char == '>' {
                    nesting -= 1
                } else if char == ',' {
                    args.push(Self::Type(record));
                    record = String::new();
                } else if nesting == 0 {
                    record.push(char);
                }
                anyhow::ensure!(nesting >= 0);
            }

            Ok(Self::List {
                pre: pre.to_string(),
                args,
            })
        } else {
            Ok(Self::Type(input.to_string()))
        }
    }

    fn into_tokens(
        self,
        generator: &Generator,
    ) -> Result<SharedGenericType<proc_macro2::TokenStream>> {
        match self {
            SharedGenericType::Type(ttype) => {
                Ok(SharedGenericType::Type(generator.translate_type(&ttype)?))
            }
            SharedGenericType::List { pre, args } => Ok(SharedGenericType::List {
                pre: proc_macro2::TokenStream::from_str(&pre).convert()?,
                args: args
                    .into_iter()
                    .map(|a| Self::into_tokens(a, generator))
                    .collect::<Result<Vec<_>>>()?,
            }),
        }
    }

    fn convert(self) -> Result<SharedGenericType<SharedType>> {
        match self {
            SharedGenericType::Type(ttype) => {
                Ok(SharedGenericType::Type(SharedType::from_cpp(&ttype)?))
            }

            SharedGenericType::List { pre, args } => Ok(SharedGenericType::List {
                pre: SharedType::from_cpp(&pre)?,
                args: args
                    .into_iter()
                    .map(Self::convert)
                    .collect::<Result<Vec<_>>>()?,
            }),
        }
    }
}

impl quote::ToTokens for SharedGenericType<SharedType> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            SharedGenericType::Type(ty) => tokens.extend(quote::quote!(#ty)),
            SharedGenericType::List { pre, args } => {
                tokens.extend(quote::quote!(#pre <));
                args.iter().for_each(|arg| arg.to_tokens(tokens));
                tokens.extend(quote::quote!(>));
            }
        };
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
        proc_macro2::TokenStream::from_str(&format!("value{n}")).convert()
    }
}

fn comment_to_docs(comment: &Option<String>) -> Vec<String> {
    comment
        .as_ref()
        .map(|x| x.split("\n").map(|x| format!(" {}", x.trim())).collect())
        .unwrap_or_default()
}

#[test]
fn parse_generics() -> Result<()> {
    let array1 = "std::array<float, 3>";
    let parsed = SharedGenericType::parse(array1)?;
    let converted = parsed.convert()?;
    // let tokens = converted.into_tokens()?;
    assert_eq!(quote::quote!(#converted).to_string(), "[f32; 3]");
    Ok(())
}
