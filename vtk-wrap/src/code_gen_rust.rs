use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::intermediate_representation::IRIdent;

// use crate::parse_cpp::StdFunction;

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
        let id = quote::format_ident!("{}", self.0);
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

impl crate::IRModule {
    fn identify_traits(&self) -> TokenStream {
        let mut out = TokenStream::new();

        out
    }

    fn implement_own_traits(&self) -> TokenStream {
        TokenStream::new()
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
                .map(|x| x.replace("@brief ", ""));
            let post = c.description.iter().skip_while(|x| x.contains("@brief"));
            let mut inside = false;
            let post = post.map(|x| {
                if x.contains("```") {
                    if !inside {
                        inside = true;
                        x.replace("```", "```cpp,ignore")
                    } else {
                        inside = false;
                        x.clone()
                    }
                } else {
                    x.clone()
                }
            });

            let constructor = quote::format_ident!("{}", c.constructor_binding_name());
            let constructor_comment = format!(" Creates a new [{name}] wrapped inside `vtkNew`");
            let destructor = quote::format_ident!("{}", c.destructor_binding_name());
            let get_ptr = quote::format_ident!("{}", c.get_ptr_binding_name());

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
                        Self(unsafe { &mut *#constructor() })
                    }

                    // This method is supposed to be used for testing only
                    #[cfg(test)]
                    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
                        unsafe extern "C" {
                            fn #get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
                        }
                        unsafe { #get_ptr( self.0 ) }
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
                fn #testname () {
                    // Create a new heap-allocated object behind vtkNew<..>
                    let obj = #name :: new();
                    // Store the internal pointer which now contains the vtkNew<..> pointer
                    let ptr = obj.0;
                    // Ensure that the vtkNew<..> pointer and its content are not null
                    assert!(!ptr.is_null());
                    assert!(unsafe { !obj._get_ptr().is_null() });
                    // Manually drop the object, freeing the memory and nulling the pointer
                    drop(obj);
                    // Wrap the previous pointer in new object without explicitly calling
                    // constructor. This allows us to access its contents with the defined API.
                    let new_obj = #name(ptr);
                    // Ensure that the previously created object is null
                    assert!(unsafe { new_obj._get_ptr().is_null() });
                }
            ));
        }

        output
    }
}

impl quote::ToTokens for crate::IRModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let modname = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Identify traits as exposable methods of parent classes and provide default
        // implementations.
        let traits = self.identify_traits();
        tokens.extend(quote::quote!(#traits));

        // Implement traits for classes exposed in this module.
        let implement_self = self.implement_own_traits();
        tokens.extend(quote::quote!(#implement_self));

        // Implement existing traits from other modules for classes exposed in this module

        let bindings = self.create_bindings();

        let mut output = quote::quote!();
        for class in self.classes.values() {
            if class.is_constructable() {
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
        }
        tokens.extend(quote::quote!(
            // #[allow(non_camel_case_types)]
            // pub mod #modname {
            //     #output
            // }
            #bindings
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
