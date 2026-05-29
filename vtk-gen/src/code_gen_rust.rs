use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::intermediate_representation::IRIdent;


fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

/// Returns false for IRType variants that have no Rust representation yet.
/// Methods containing unsupported types are skipped before code gen.
fn ir_type_is_supported(irtype: &crate::IRType) -> bool {
    use crate::IRType::*;
    match irtype {
        // Path types (VTK object names) are only bridgeable as opaque pointers.
        // Passing VTK objects by value or reference (e.g. `const vtkStdString&`) is not bridgeable.
        FileMode | File | Path(_) => false,
        Pointer(inner) => {
            // Mutable char* (Pointer(c_char)) is not safely bridgeable: it may be a signed
            // char data buffer rather than a C string, causing char*/signed char* type errors.
            // Only Pointer(Const(c_char)) (= const char*, C string) and VTK object pointers are ok.
            if matches!(inner.as_ref(), c_char) {
                return false;
            }
            is_string_type(irtype) || matches!(inner.as_ref(), Path(_))
        }
        // Heap-allocated collection types cannot cross the FFI boundary safely.
        Vec(_) | LinkedList(_) | Map(_, _) => false,
        Ref(inner) | Const(inner) => ir_type_is_supported(inner),
        Array(inner, _) => ir_type_is_supported(inner),
        _ => true,
    }
}

fn method_is_supported(method: &crate::IRMethod) -> bool {
    // std::string return types can't be bridged: the C++ callee returns by value and the
    // resulting const char* would dangle immediately. String *parameters* are fine.
    !matches!(method.return_type, crate::IRType::String)
        && ir_type_is_supported(&method.return_type)
        && method.args.iter().all(|(_, irtype)| ir_type_is_supported(irtype))
}

/// Returns a safe Rust identifier by appending an underscore if the given string is a Rust keyword.
fn safe_ident(s: &str) -> String {
    if is_rust_keyword(s) {
        format!("{}_", s)
    } else {
        s.to_string()
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn is_string_type(irtype: &crate::IRType) -> bool {
    use crate::IRType::*;
    match irtype {
        String | c_char => true,
        Const(inner) | Ref(inner) | Pointer(inner) => is_string_type(inner),
        _ => false,
    }
}

/// Rust type used in the trait method signature (user-facing).
fn ir_type_as_param_sig(irtype: &crate::IRType) -> TokenStream {
    if is_string_type(irtype) {
        return quote::quote!(&str);
    }
    match irtype {
        // VTK object pointers are always passed as opaque c_void pointers across the C bridge.
        crate::IRType::Path(_) => quote::quote!(*mut core::ffi::c_void),
        crate::IRType::Pointer(inner) if matches!(inner.as_ref(), crate::IRType::Path(_)) => {
            quote::quote!(*mut core::ffi::c_void)
        }
        _ => quote::quote!(#irtype),
    }
}

/// Rust type used in the extern "C" declaration inside an impl body.
fn ir_type_as_c_extern(irtype: &crate::IRType) -> TokenStream {
    match irtype {
        crate::IRType::String => quote::quote!(*const core::ffi::c_char),
        crate::IRType::Const(inner) if matches!(inner.as_ref(), crate::IRType::String) => {
            quote::quote!(*const core::ffi::c_char)
        }
        crate::IRType::Path(_) => quote::quote!(*mut core::ffi::c_void),
        crate::IRType::Pointer(inner) if matches!(inner.as_ref(), crate::IRType::Path(_)) => {
            quote::quote!(*mut core::ffi::c_void)
        }
        other => quote::quote!(#other),
    }
}

fn build_method_impl(class_name: &str, method: &crate::IRMethod) -> TokenStream {
    use quote::quote;

    let short = safe_ident(&method.short_name(class_name));
    let method_ident = quote::format_ident!("{}", short);
    let full_name_ident = quote::format_ident!("{}", &method.name);

    let ret_sig = ir_type_as_param_sig(&method.return_type);
    let ret_c = ir_type_as_c_extern(&method.return_type);
    let is_string_ret = is_string_type(&method.return_type);
    let is_void_ret = matches!(&method.return_type, crate::IRType::Unit);

    let mut pre_call: Vec<TokenStream> = vec![];
    let mut extern_args: Vec<TokenStream> = vec![quote!(sself: *mut core::ffi::c_void)];
    let mut trait_params: Vec<TokenStream> = vec![];
    let mut call_args: Vec<TokenStream> = vec![quote!(self.0)];

    for (name, irtype) in &method.args {
        let param_ty = ir_type_as_param_sig(irtype);
        trait_params.push(quote!(#name: #param_ty));

        if is_string_type(irtype) {
            let c_name = quote::format_ident!("c_{}", name.0);
            pre_call.push(quote!(
                let #c_name = std::ffi::CString::new(#name).expect("CString::new failed");
            ));
            extern_args.push(quote!(#name: *const core::ffi::c_char));
            call_args.push(quote!(#c_name.as_ptr()));
        } else {
            let c_type = ir_type_as_c_extern(irtype);
            extern_args.push(quote!(#name: #c_type));
            call_args.push(quote!(#name));
        }
    }

    let body = if is_string_ret {
        quote!(
            #(#pre_call)*
            unsafe extern "C" {
                fn #full_name_ident(#(#extern_args),*) -> *const core::ffi::c_char;
            }
            let ptr = unsafe { #full_name_ident(#(#call_args),*) };
            if ptr.is_null() { return ""; }
            unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
        )
    } else if is_void_ret {
        quote!(
            #(#pre_call)*
            unsafe extern "C" {
                fn #full_name_ident(#(#extern_args),*);
            }
            unsafe { #full_name_ident(#(#call_args),*) }
        )
    } else {
        quote!(
            #(#pre_call)*
            unsafe extern "C" {
                fn #full_name_ident(#(#extern_args),*) -> #ret_c;
            }
            unsafe { #full_name_ident(#(#call_args),*) }
        )
    };

    quote!(
        fn #method_ident(&mut self, #(#trait_params),*) -> #ret_sig {
            #body
        }
    )
}

impl ToTokens for crate::IRType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use crate::IRType::*;
        use quote::quote;
        // match self {
        //     Const(_) => (),
        //     _ => tokens.extend(quote!(mut)),
        // }

        let ty = match self {
            Unit => quote::quote!(()),
            c_char => quote!(core::ffi::c_char),
            c_signed_char => quote!(core::ffi::c_schar),
            c_short => quote!(core::ffi::c_short),
            c_int => quote!(core::ffi::c_int),
            c_long => quote!(core::ffi::c_long),
            c_longlong => quote!(core::ffi::c_longlong),
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
        let name = if is_rust_keyword(&self.0) {
            format!("{}_", self.0)
        } else {
            self.0.clone()
        };
        let id = quote::format_ident!("{}", name);
        tokens.extend(quote::quote!(#id))
    }
}

impl ToTokens for crate::IRMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate::IRMethod {
            name,
            return_type,
            args,
            ..
        } = &self;

        let name = quote::format_ident!("{name}");
        let args = args.iter().map(|(name, ty)| quote::quote!(#name: #ty));
        let code = quote::quote!(fn #name (#(#args),*) -> #return_type;);

        tokens.extend(code);
    }
}

impl crate::IRModule {
    fn identify_traits(&self) -> TokenStream {
        let mut output = TokenStream::new();
        for (class_name, ir_struct) in &self.classes {
            if ir_struct.exposable_methods.is_empty() {
                continue;
            }
            let trait_ident = quote::format_ident!("{}", capitalize_first(class_name));

            // Supertrait bounds would require implementing every ancestor trait for each
            // concrete struct across module boundaries, which the generator doesn't yet support.
            let parent_bounds: Vec<TokenStream> = vec![];

            let method_sigs: Vec<TokenStream> = ir_struct
                .exposable_methods
                .iter()
                .filter(|m| method_is_supported(m))
                .map(|method| {
                    let short = safe_ident(&method.short_name(class_name));
                    let method_ident = quote::format_ident!("{}", short);
                    let args: Vec<TokenStream> = method
                        .args
                        .iter()
                        .map(|(name, irtype)| {
                            let sig_ty = ir_type_as_param_sig(irtype);
                            quote::quote!(#name: #sig_ty)
                        })
                        .collect();
                    let ret = ir_type_as_param_sig(&method.return_type);
                    quote::quote!(fn #method_ident(&mut self, #(#args),*) -> #ret;)
                })
                .collect();

            let trait_def = if parent_bounds.is_empty() {
                quote::quote!(
                    pub trait #trait_ident {
                        #(#method_sigs)*
                    }
                )
            } else {
                quote::quote!(
                    pub trait #trait_ident: #(#parent_bounds)+* {
                        #(#method_sigs)*
                    }
                )
            };

            output.extend(trait_def);
        }
        output
    }

    fn implement_own_traits(&self) -> TokenStream {
        let mut output = TokenStream::new();
        for (class_name, ir_struct) in &self.classes {
            if !ir_struct.is_constructable() || ir_struct.exposable_methods.is_empty() {
                continue;
            }
            let struct_ident = quote::format_ident!("{}", class_name);
            let trait_ident = quote::format_ident!("{}", capitalize_first(class_name));

            let methods: Vec<TokenStream> = ir_struct
                .exposable_methods
                .iter()
                .filter(|m| method_is_supported(m))
                .map(|method| build_method_impl(class_name, method))
                .collect();

            output.extend(quote::quote!(
                impl #trait_ident for #struct_ident {
                    #(#methods)*
                }
            ));
        }
        output
    }

    fn create_bindings(&self) -> TokenStream {
        let mut output = TokenStream::new();
        for c in self.classes.values().filter(|c| c.is_constructable()) {
            let struct_name = &c.name;
            let name = quote::format_ident!("{}", c.name);

            let pre = c
                .description
                .iter()
                .take_while(|x| x.contains("@brief"))
                .map(|x| format!(" {}", x.replace("@brief ", "").trim()));
            let post = c.description.iter().skip_while(|x| x.contains("@brief"));
            let mut inside = false;
            let post = post.map(|x| {
                let x = if x.contains("```") {
                    if !inside {
                        inside = true;
                        x.replace("```", "```cpp,ignore")
                    } else {
                        inside = false;
                        x.clone()
                    }
                } else {
                    x.clone()
                };
                format!(" {}", x.trim())
            });

            let constructor = quote::format_ident!("{}", c.constructor_binding_name());
            let constructor_comment = format!(" Creates a new [{name}] via `{struct_name}::New()`");
            let destructor = quote::format_ident!("{}", c.destructor_binding_name());

            let testname = quote::format_ident!("test_{}_create_drop", c.name);

            output.extend(quote::quote!(
                #(#[doc = #pre])*
                #[doc = ""]
                #(#[doc = #post])*
                #[allow(non_camel_case_types)]
                pub struct #name(*mut core::ffi::c_void);

                impl #name {
                    #[doc = #constructor_comment]
                    #[doc(alias = #struct_name)]
                    pub fn new() -> Self {
                        unsafe extern "C" {
                            fn #constructor() -> *mut core::ffi::c_void;
                        }
                        Self(unsafe { #constructor() })
                    }
                }

                impl std::default::Default for #name {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                impl Drop for #name {
                    fn drop(&mut self) {
                        unsafe extern "C" {
                            fn #destructor(sself: *mut core::ffi::c_void);
                        }
                        unsafe { #destructor(self.0) }
                        self.0 = core::ptr::null_mut();
                    }
                }

                #[test]
                fn #testname() {
                    let obj = #name::new();
                    assert!(!obj.0.is_null());
                    drop(obj);
                }
            ));
        }

        output
    }
}

impl quote::ToTokens for crate::IRModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let traits = self.identify_traits();
        tokens.extend(quote::quote!(#traits));

        let implement_self = self.implement_own_traits();
        tokens.extend(quote::quote!(#implement_self));

        let bindings = self.create_bindings();
        tokens.extend(bindings);
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

#[cfg(test)]
mod gen_rust_tests {
    use super::*;
    use crate::intermediate_representation::{IRIdent, IRMethod, IRModule, IRStruct, IRType};
    use std::collections::BTreeMap;

    fn make_module(classes: BTreeMap<String, IRStruct>) -> IRModule {
        IRModule {
            name: "TestModule".to_string(),
            classes,
        }
    }

    #[test]
    fn test_is_rust_keyword_true_for_keywords() {
        for kw in &["type", "self", "break", "fn", "impl", "trait", "use", "ref", "unsafe"] {
            assert!(is_rust_keyword(kw), "{kw} should be a keyword");
        }
    }

    #[test]
    fn test_is_rust_keyword_false_for_non_keywords() {
        for word in &["radius", "vtkFoo", "set_value", "get_output", ""] {
            assert!(!is_rust_keyword(word), "{word} should not be a keyword");
        }
    }

    #[test]
    fn test_safe_ident_appends_underscore_for_keywords() {
        assert_eq!(safe_ident("type"), "type_");
        assert_eq!(safe_ident("self"), "self_");
        assert_eq!(safe_ident("break"), "break_");
        assert_eq!(safe_ident("ref"), "ref_");
    }

    #[test]
    fn test_safe_ident_unchanged_for_non_keywords() {
        assert_eq!(safe_ident("radius"), "radius");
        assert_eq!(safe_ident("get_output"), "get_output");
    }

    #[test]
    fn test_capitalize_first_vtk_class() {
        assert_eq!(capitalize_first("vtkFoo"), "VtkFoo");
        assert_eq!(capitalize_first("vtkObjectBase"), "VtkObjectBase");
    }

    #[test]
    fn test_capitalize_first_edge_cases() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
    }

    #[test]
    fn test_ir_type_is_supported_rejects_filemode() {
        assert!(!ir_type_is_supported(&IRType::FileMode));
        assert!(!ir_type_is_supported(&IRType::File));
        assert!(!ir_type_is_supported(&IRType::Pointer(Box::new(IRType::FileMode))));
    }

    #[test]
    fn test_ir_type_is_supported_accepts_primitives() {
        assert!(ir_type_is_supported(&IRType::c_int));
        assert!(ir_type_is_supported(&IRType::double));
        assert!(ir_type_is_supported(&IRType::bool));
        assert!(ir_type_is_supported(&IRType::String));
        assert!(ir_type_is_supported(&IRType::Unit));
    }

    #[test]
    fn test_ir_ident_keyword_gets_underscore_suffix() {
        let id = IRIdent("type".to_string());
        assert_eq!(quote::quote!(#id).to_string(), "type_");
    }

    #[test]
    fn test_ir_ident_self_gets_underscore_suffix() {
        let id = IRIdent("self".to_string());
        assert_eq!(quote::quote!(#id).to_string(), "self_");
    }

    #[test]
    fn test_ir_ident_normal_name_unchanged() {
        let id = IRIdent("radius".to_string());
        assert_eq!(quote::quote!(#id).to_string(), "radius");
    }

    #[test]
    fn test_identify_traits_empty_module_produces_no_tokens() {
        let module = make_module(BTreeMap::new());
        assert!(module.identify_traits().is_empty());
    }

    #[test]
    fn test_identify_traits_skips_class_with_no_methods() {
        let mut classes = BTreeMap::new();
        classes.insert(
            "vtkFoo".to_string(),
            IRStruct::test_new("vtkFoo", vec!["vtkObjectBase"], vec![]),
        );
        let module = make_module(classes);
        assert!(module.identify_traits().is_empty());
    }

    #[test]
    fn test_identify_traits_generates_trait_for_class_with_methods() {
        let mut classes = BTreeMap::new();
        classes.insert(
            "vtkFoo".to_string(),
            IRStruct::test_new(
                "vtkFoo",
                vec!["vtkObjectBase"],
                vec![IRMethod::test_new("vtk_foo_update", "Update", IRType::Unit, vec![])],
            ),
        );
        let module = make_module(classes);
        let output = module.identify_traits().to_string();
        assert!(output.contains("VtkFoo"), "trait name VtkFoo missing");
        assert!(output.contains("update"), "method name update missing");
    }

    #[test]
    fn test_identify_traits_keyword_method_name_gets_suffix() {
        let mut classes = BTreeMap::new();
        classes.insert(
            "vtkBreakPoint".to_string(),
            IRStruct::test_new(
                "vtkBreakPoint",
                vec!["vtkObjectBase"],
                vec![IRMethod::test_new(
                    "vtk_break_point_break",
                    "Break",
                    IRType::Unit,
                    vec![],
                )],
            ),
        );
        let module = make_module(classes);
        let output = module.identify_traits().to_string();
        assert!(output.contains("break_"), "keyword method should be renamed to break_");
        assert!(!output.contains(" break ("), "bare keyword 'break' must not appear as method name");
    }

    #[test]
    fn test_implement_own_traits_skips_non_constructable() {
        let mut classes = BTreeMap::new();
        // no "vtkObjectBase" parent -> not constructable
        classes.insert(
            "vtkAbstract".to_string(),
            IRStruct::test_new(
                "vtkAbstract",
                vec![],
                vec![IRMethod::test_new("vtk_abstract_run", "Run", IRType::Unit, vec![])],
            ),
        );
        let module = make_module(classes);
        assert!(module.implement_own_traits().is_empty());
    }

    #[test]
    fn test_implement_own_traits_generates_impl_for_constructable() {
        let mut classes = BTreeMap::new();
        classes.insert(
            "vtkFoo".to_string(),
            IRStruct::test_new(
                "vtkFoo",
                vec!["vtkObjectBase"],
                vec![IRMethod::test_new(
                    "vtk_foo_set_value",
                    "SetValue",
                    IRType::Unit,
                    vec![(IRIdent("v".to_string()), IRType::c_int)],
                )],
            ),
        );
        let module = make_module(classes);
        let output = module.implement_own_traits().to_string();
        assert!(output.contains("impl VtkFoo for vtkFoo"));
        assert!(output.contains("vtk_foo_set_value"));
        assert!(output.contains("set_value"));
    }
}
