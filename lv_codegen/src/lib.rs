use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::{ToTokens, format_ident};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use syn::{FnArg, ForeignItem, ForeignItemFn, Item, ReturnType, TypePath, parse_str};
use thiserror::Error;

type CGResult<T> = Result<T, Box<dyn Error>>;

const LIB_PREFIX: &str = "lv_";

#[cfg(feature = "no_ecs")]
const FUNCTION_BLACKLIST: [&str; 9] = [
    "lv_style_init",                   // use Style::default() instead
    "lv_obj_null_on_delete",           // can invalidate NonNull<>
    "lv_obj_add_style",                // use functions::lv_obj_add_style() instead
    "lv_obj_set_parent",               // use functions::lv_obj_set_parent() instead
    "lv_obj_add_event_cb",             // implemented manually
    "lv_event_get_target",             // use functions::lv_event_get_target() instead
    "lv_event_get_target_obj",         // use functions::lv_event_get_target_obj() instead
    "lv_event_get_current_target_obj", // use functions::lv_event_get_current_target_obj() instead
    "lv_list_get_button_text",         // lifetime can't be elided
];

#[cfg(not(feature = "no_ecs"))]
const FUNCTION_BLACKLIST: [&str; 12] = [
    "lv_obj_null_on_delete",           // can invalidate NonNull<>
    "lv_obj_add_style",                // add component instead
    "lv_obj_replace_style",            // replace component instead
    "lv_obj_remove_style",             // remove component instead
    "lv_obj_remove_style_all",         // remove components instead
    "lv_obj_set_parent",               // use EntityWorldMut::add_child() instead
    "lv_obj_add_event_cb",             // implemented manually
    "lv_style_init",                   // use Style::default() instead
    "lv_event_get_target",             // use functions::lv_event_get_target() instead
    "lv_event_get_target_obj",         // use functions::lv_event_get_target_obj() instead
    "lv_event_get_current_target_obj", // use functions::lv_event_get_current_target_obj() instead
    "lv_list_get_button_text",         // lifetime can't be elided
];

#[derive(Debug, Clone, Error)]
pub enum SkipReason {
    #[error("Return value is array ({0})")]
    ReturnArray(String),
    #[error("Array as argument ({0})")]
    ArrayArgument(String),
    #[error("Void pointer as argument ({0})")]
    VoidPtrArgument(String),
    #[error("Already implemented ({0})")]
    CustomStruct(String),
    #[error("Constructor function ({0})")]
    Constructor(String),
    #[error("Blacklisted function ({0})")]
    Blacklisted(String),
}

pub type WrapperResult<T> = Result<T, SkipReason>;

pub trait Rusty {
    type Parent;

    fn code(&self, parent: &Self::Parent) -> WrapperResult<TokenStream>;
}

#[derive(Clone)]
pub struct LvWidget {
    name: String,
    methods: Vec<LvFunc>,
}

impl Rusty for LvWidget {
    type Parent = ();

    fn code(&self, _parent: &Self::Parent) -> WrapperResult<TokenStream> {
        let methods: Vec<TokenStream> = self
            .methods
            .iter()
            .flat_map(|m| {
                m.code(self).map_err(|error| match error {
                    SkipReason::ReturnArray(_)
                    | SkipReason::ArrayArgument(_)
                    | SkipReason::VoidPtrArgument(_) => println!("{} - {}", m.name, error),
                    _ => {}
                })
            })
            .collect();
        Ok(quote! {
            #(#methods)*
        })
    }
}

impl LvWidget {
    pub fn gen_impl(&self) -> WrapperResult<TokenStream> {
        let pascal_name = format_ident!("{}", to_pascal_case(&self.name));
        let create_function = format_ident!("lv_{}_create", &self.name);

        if self.name == "obj" || self.name == "style" {
            Err(SkipReason::CustomStruct(self.name.clone()))
        } else {
            Ok(quote! {
                impl_widget!(#pascal_name, lightvgl_sys::#create_function);
            })
        }
    }
}

#[derive(Clone)]
pub struct LvFunc {
    name: String,
    args: Vec<LvArg>,
    ret: Option<LvType>,
}

impl LvFunc {
    pub fn new(name: String, args: Vec<LvArg>, ret: Option<LvType>) -> Self {
        Self { name, args, ret }
    }

    pub fn is_method(&self) -> bool {
        if !self.args.is_empty() {
            let first_arg = &self.args[0];
            return first_arg.typ.literal_name.contains("lv_obj_t")
                || first_arg.typ.literal_name.contains("lv_style_t");
        }
        false
    }
}

impl Rusty for LvFunc {
    type Parent = LvWidget;

    fn code(&self, parent: &Self::Parent) -> WrapperResult<TokenStream> {
        let templ = format!("{}{}_", LIB_PREFIX, parent.name.as_str());
        let new_name = self.name.replace(templ.as_str(), "");
        let func_name = format_ident!("{}", self.name);

        // skip constructors and blacklisted functions
        if new_name.as_str().eq("create") && parent.name != "obj" {
            return Err(SkipReason::Constructor(self.name.clone()));
        } else if FUNCTION_BLACKLIST.contains(&self.name.as_str()) {
            return Err(SkipReason::Blacklisted(self.name.clone()));
        }

        // Handle return values
        let return_tokens = match self.ret {
            // function returns void
            None => quote!(),
            // function returns something
            _ => {
                let return_value: &LvType = self.ret.as_ref().unwrap();
                if !return_value.is_pointer() {
                    parse_str(&format!("-> {}", return_value.literal_name)).unwrap_or_else(|_| {
                        panic!("Cannot parse {} as type", return_value.literal_name)
                    })
                } else {
                    if return_value.is_const_native_object() || return_value.is_mut_native_object()
                    {
                        parse_str("-> Option<Wdg>").unwrap()
                    } else if return_value.is_const_str() || return_value.is_mut_str() {
                        parse_str("-> &CStr").unwrap()
                    } else if return_value.is_array() {
                        return Err(SkipReason::ReturnArray(return_value.literal_name.clone()));
                    } else if return_value.is_mut_pointer() {
                        parse_str(&format!("-> Option<NonNull<{}>>", return_value.raw_name()))
                            .unwrap()
                    } else {
                        parse_str(&format!("-> Option<&{}>", return_value.raw_name())).unwrap()
                    }
                }
            }
        };

        // Make sure all arguments can be generated, skip the first arg (self)!
        for arg in self.args.iter().skip(1) {
            arg.code(self)?;
        }

        // Generate the arguments being passed into the Rust 'wrapper'
        //
        // - Iif the first argument (of the C function) is const then we require a &self immutable reference, otherwise an &mut self reference
        // - The arguments will be appended to the accumulator (args_accumulator) as they are generated in the closure
        let args_decl = self.args.iter().fold(quote!(), |args_accumulator, arg| {
            if let Ok(next_arg) = arg.code(self) {
                // If the accummulator is empty then we call quote! only with the next_arg content
                if args_accumulator.is_empty() {
                    quote! {#next_arg}
                }
                // Otherwise we append next_arg at the end of the accumulator
                else {
                    quote! {#args_accumulator, #next_arg}
                }
            } else {
                args_accumulator
            }
        });

        let args_preprocessing = self
            .args
            .iter()
            .enumerate()
            .fold(quote!(), |args, (i, arg)| {
                // if first arg is `const`, then it should be immutable
                let next_arg = if i == 0 {
                    quote!()
                } else {
                    let var = arg.get_preprocessing();
                    quote!(#var)
                };
                if args.is_empty() {
                    quote! {
                        #next_arg
                    }
                } else {
                    quote! {
                        #args
                        #next_arg
                    }
                }
            });

        let args_postprocessing = self
            .args
            .iter()
            .enumerate()
            .fold(quote!(), |args, (i, arg)| {
                // if first arg is `const`, then it should be immutable
                let next_arg = if i == 0 {
                    quote!()
                } else {
                    let var = arg.get_postprocessing();
                    quote!(#var)
                };
                if args.is_empty() {
                    quote! {
                        #next_arg
                    }
                } else {
                    quote! {
                        #args
                        #next_arg
                    }
                }
            });

        // Generate the arguments being passed into the FFI interface
        //
        // - The first argument will be always self.core.raw().as_mut() (see quote! when arg_idx == 0), it's most likely a pointer to lv_obj_t
        //   TODO: When handling getters this should be self.raw().as_ptr() instead, this also requires updating args_decl
        // - The arguments will be appended to the accumulator (args_accumulator) as they are generated in the closure
        let ffi_args = self.args.iter().fold(quote!(), |args_accumulator, arg| {
            let next_arg = if arg.typ.is_mut_native_object() {
                let var = arg.get_value_usage();
                quote! {#var.raw_mut()}
            } else if arg.typ.is_const_native_object() {
                let var = arg.get_value_usage();
                quote! {#var.raw()}
            } else {
                let var = arg.get_value_usage();
                quote!(#var)
            };

            // If the accummulator is empty then we call quote! only with the next_arg content
            if args_accumulator.is_empty() {
                quote! {#next_arg}
            }
            // Otherwise we append next_arg at the end of the accumulator
            else {
                quote! {#args_accumulator, #next_arg}
            }
        });

        let return_assignment;
        let return_expr;
        let optional_semicolon;
        let return_value = self.ret.as_ref();
        if let Some(return_value) = return_value {
            if return_value.is_const_native_object() || return_value.is_mut_native_object() {
                return_assignment = quote!(let pointer = );
                return_expr = quote!(Wdg::try_from_ptr(pointer));
                optional_semicolon = quote!(;);
            } else if return_value.is_const_str() || return_value.is_mut_str() {
                return_assignment = quote!(let pointer = );
                return_expr = quote!(CStr::from_ptr(pointer));
                optional_semicolon = quote!(;);
            } else if return_value.is_pointer() {
                if return_value.is_mut_pointer() {
                    return_assignment = quote!(let pointer = );
                    return_expr = quote!(NonNull::new(pointer));
                    optional_semicolon = quote!(;);
                } else {
                    return_assignment = quote!(let pointer = );
                    return_expr = quote!(if !pointer.is_null() {
                        Some(&*pointer)
                    } else {
                        None
                    });
                    optional_semicolon = quote!(;);
                }
            } else {
                if args_postprocessing.is_empty() {
                    return_assignment = quote!();
                    return_expr = quote!();
                    optional_semicolon = quote!();
                } else {
                    return_assignment = quote!(let rust_result = );
                    return_expr = quote!(rust_result);
                    optional_semicolon = quote!(;);
                }
            }
        } else {
            if args_postprocessing.is_empty() {
                return_assignment = quote!();
                return_expr = quote!();
                optional_semicolon = quote!();
            } else {
                return_assignment = quote!(let rust_result = );
                return_expr = quote!(rust_result);
                optional_semicolon = quote!(;);
            }
        }

        Ok(quote! {
            pub fn #func_name(#args_decl) #return_tokens {
                unsafe {
                    #args_preprocessing
                    #return_assignment lightvgl_sys::#func_name(#ffi_args)#optional_semicolon
                    #args_postprocessing
                    #return_expr
                }
            }
        })
    }
}

impl From<ForeignItemFn> for LvFunc {
    fn from(ffi: ForeignItemFn) -> Self {
        let ret = match ffi.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, typ) => Some(typ.into()),
        };
        Self::new(
            ffi.sig.ident.to_string(),
            ffi.sig
                .inputs
                .iter()
                .filter_map(|fa| {
                    // Since we know those are foreign functions, we only care about typed arguments
                    if let FnArg::Typed(tya) = fa {
                        Some(tya)
                    } else {
                        None
                    }
                })
                .map(|a| a.clone().into())
                .collect::<Vec<LvArg>>(),
            ret,
        )
    }
}

#[derive(Clone)]
pub struct LvArg {
    name: String,
    typ: LvType,
}

impl From<syn::PatType> for LvArg {
    fn from(fa: syn::PatType) -> Self {
        Self::new(fa.pat.to_token_stream().to_string(), fa.ty.into())
    }
}

impl LvArg {
    pub fn new(name: String, typ: LvType) -> Self {
        Self { name, typ }
    }

    pub fn get_name_ident(&self) -> Ident {
        // Filter Rust language keywords
        syn::parse_str::<syn::Ident>(self.name.as_str())
            .unwrap_or_else(|_| format_ident!("r#{}", self.name.as_str()))
    }

    pub fn get_preprocessing(&self) -> TokenStream {
        // TODO: A better way to handle this, instead of `is_sometype()`, is using the Rust
        //       type system itself.

        if self.get_type().is_mut_str() {
            // Convert CString to *mut i8
            let name = format_ident!("{}", &self.name);
            let name_raw = format_ident!("{}_raw", &self.name);
            quote! {
                let #name_raw = #name.clone().into_raw();
            }
        } else {
            quote! {}
        }
    }

    pub fn get_postprocessing(&self) -> TokenStream {
        if self.get_type().is_mut_str() {
            // Convert *mut i8 back to CString
            let name = format_ident!("{}", &self.name);
            let name_raw = format_ident!("{}_raw", &self.name);
            quote! {
                *#name = ::alloc::ffi::CString::from_raw(#name_raw);
            }
        } else {
            quote! {}
        }
    }

    pub fn get_value_usage(&self) -> TokenStream {
        let ident = self.get_name_ident();
        if self.typ.is_const_str() {
            quote! {
                #ident.as_ptr()
            }
        } else if self.typ.is_mut_str() {
            let ident_raw = format_ident!("{}_raw", &ident);
            quote! {
                #ident_raw
            }
        } else if self.typ.is_const_style() {
            quote! {
                #ident.raw()
            }
        } else if self.typ.is_mut_style() {
            quote! {
                #ident.raw_mut()
            }
        } else {
            quote! {
                #ident
            }
        }
    }

    pub fn get_type(&self) -> &LvType {
        &self.typ
    }
}

impl Rusty for LvArg {
    type Parent = LvFunc;

    fn code(&self, _parent: &Self::Parent) -> WrapperResult<TokenStream> {
        let name = self.get_name_ident();
        let typ = self.typ.code(self)?;
        Ok(quote! {
            #name: #typ
        })
    }
}

#[derive(Clone)]
pub struct LvType {
    literal_name: String,
    _r_type: Option<Box<syn::Type>>,
}

impl LvType {
    pub fn new(literal_name: String) -> Self {
        Self {
            literal_name,
            _r_type: None,
        }
    }

    pub fn from(r_type: Box<syn::Type>) -> Self {
        Self {
            literal_name: r_type.to_token_stream().to_string(),
            _r_type: Some(r_type),
        }
    }

    pub fn raw_name(&self) -> String {
        self.literal_name
            .replace("* const ", "")
            .replace("* mut ", "")
    }

    pub fn is_const(&self) -> bool {
        self.literal_name.starts_with("const ")
    }

    pub fn is_const_str(&self) -> bool {
        self.literal_name == "* const :: core :: ffi :: c_char"
    }

    pub fn is_mut_str(&self) -> bool {
        self.literal_name == "* mut :: core :: ffi :: c_char"
    }

    pub fn is_const_native_object(&self) -> bool {
        self.literal_name == "* const lv_obj_t" || self.literal_name == "* const _lv_obj_t"
    }

    pub fn is_mut_native_object(&self) -> bool {
        self.literal_name == "* mut lv_obj_t" || self.literal_name == "* mut _lv_obj_t"
    }

    pub fn is_mut_style(&self) -> bool {
        self.literal_name == "* mut lv_style_t"
    }

    pub fn is_const_style(&self) -> bool {
        self.literal_name == "* const lv_style_t"
    }

    pub fn is_pointer(&self) -> bool {
        self.literal_name.starts_with('*')
    }

    pub fn is_const_pointer(&self) -> bool {
        self.literal_name.starts_with("* const")
    }

    pub fn is_mut_pointer(&self) -> bool {
        self.literal_name.starts_with("* mut")
    }

    pub fn is_array(&self) -> bool {
        self.literal_name.starts_with("* mut *") || self.literal_name.starts_with("* const *")
    }
}

impl Rusty for LvType {
    type Parent = LvArg;

    fn code(&self, _parent: &Self::Parent) -> WrapperResult<TokenStream> {
        let val = if self.is_array() {
            return Err(SkipReason::ArrayArgument(self.literal_name.clone()));
        } else if self.is_const_str() {
            quote!(&CStr)
        } else if self.is_mut_str() {
            quote!(&mut ::alloc::ffi::CString)
        } else if self.is_const_native_object() {
            quote!(&crate::widgets::Wdg)
        } else if self.is_mut_native_object() {
            quote!(&mut crate::widgets::Wdg)
        } else if self.is_const_style() {
            quote!(&crate::styles::Style)
        } else if self.is_mut_style() {
            quote!(&mut crate::styles::Style)
        } else {
            let raw_name = self.raw_name();
            if raw_name == "cty :: c_void" {
                return Err(SkipReason::VoidPtrArgument(self.literal_name.clone()));
            }
            let ty: TypePath = parse_str(&raw_name)
                .unwrap_or_else(|_| panic!("Cannot parse {raw_name} to a type"));
            if self.literal_name.starts_with("* mut") {
                quote!(&mut #ty)
            } else if self.literal_name.starts_with("*") {
                quote!(&#ty)
            } else {
                quote!(#ty)
            }
        };

        Ok(val)
    }
}

impl From<Box<syn::Type>> for LvType {
    fn from(t: Box<syn::Type>) -> Self {
        Self::from(t)
    }
}

pub struct CodeGen {
    functions: Vec<LvFunc>,
    widgets: Vec<LvWidget>,
}

impl CodeGen {
    pub fn from(code: &str) -> CGResult<Self> {
        let functions = Self::load_func_defs(code)?;
        let widgets = Self::extract_widgets(&functions)?;
        Ok(Self { functions, widgets })
    }

    pub fn get_widgets(&self) -> &Vec<LvWidget> {
        &self.widgets
    }

    fn extract_widgets(functions: &[LvFunc]) -> CGResult<Vec<LvWidget>> {
        let widget_names = Self::get_widget_names(functions);

        let widgets = functions.iter().fold(HashMap::new(), |mut ws, f| {
            for widget_name in &widget_names {
                if f.name
                    .starts_with(format!("{}{}_", LIB_PREFIX, widget_name).as_str())
                    && f.is_method()
                {
                    ws.entry(widget_name.clone())
                        .or_insert_with(|| LvWidget {
                            name: widget_name.clone(),
                            methods: Vec::new(),
                        })
                        .methods
                        .push(f.clone())
                }
            }
            ws
        });

        Ok(widgets.values().cloned().collect())
    }

    fn get_widget_names(functions: &[LvFunc]) -> Vec<String> {
        let reg = format!("^{}([^_]+)_(create|init)$", LIB_PREFIX);
        let create_func = Regex::new(reg.as_str()).unwrap();

        functions
            .iter()
            .filter(|e| (create_func.is_match(e.name.as_str())) && e.args.len() == 1)
            .map(|f| {
                String::from(
                    create_func
                        .captures(f.name.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn load_func_defs(bindgen_code: &str) -> CGResult<Vec<LvFunc>> {
        let ast: syn::File = syn::parse_str(bindgen_code)?;
        let fns = ast
            .items
            .into_iter()
            .filter_map(|e| {
                if let Item::ForeignMod(fm) = e {
                    Some(fm)
                } else {
                    None
                }
            })
            .flat_map(|e| {
                e.items.into_iter().filter_map(|it| {
                    if let ForeignItem::Fn(f) = it {
                        Some(f)
                    } else {
                        None
                    }
                })
            })
            .filter(|ff| ff.sig.ident.to_string().starts_with(LIB_PREFIX))
            .map(|ff| ff.into())
            .collect::<Vec<LvFunc>>();
        Ok(fns)
    }

    pub fn get_function_names(&self) -> CGResult<Vec<String>> {
        Ok(self.functions.iter().map(|f| f.name.clone()).collect())
    }
}

#[cfg(test)]
mod test {
    use crate::{CodeGen, LvArg, LvFunc, LvType, LvWidget, Rusty};
    use itertools::Itertools;
    use quote::quote;

    #[test]
    fn can_load_bindgen_fns() {
        let bindgen_code = quote! {
            extern "C" {
                #[doc = " Return with the screen of an object"]
                #[doc = " @param obj pointer to an object"]
                #[doc = " @return pointer to a screen"]
                pub fn lv_obj_get_screen(obj: *const lv_obj_t) -> *mut lv_obj_t;
            }
        };

        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let ffn = cg.get(0).unwrap();
        assert_eq!(ffn.name, "lv_obj_get_screen");
        assert_eq!(ffn.args[0].name, "obj");
    }

    #[test]
    fn can_identify_widgets_from_function_names() {
        let funcs = vec![
            LvFunc::new(
                "lv_obj_create".to_string(),
                vec![LvArg::new(
                    "parent".to_string(),
                    LvType::new("abc".to_string()),
                )],
                None,
            ),
            LvFunc::new(
                "lv_btn_create".to_string(),
                vec![LvArg::new(
                    "parent".to_string(),
                    LvType::new("abc".to_string()),
                )],
                None,
            ),
            LvFunc::new(
                "lv_do_something".to_string(),
                vec![LvArg::new(
                    "parent".to_string(),
                    LvType::new("abc".to_string()),
                )],
                None,
            ),
            LvFunc::new(
                "lv_invalid_create".to_string(),
                vec![
                    LvArg::new("parent".to_string(), LvType::new("abc".to_string())),
                    LvArg::new("copy_from".to_string(), LvType::new("bcf".to_string())),
                ],
                None,
            ),
            LvFunc::new(
                "lv_cb_create".to_string(),
                vec![LvArg::new(
                    "parent".to_string(),
                    LvType::new("abc".to_string()),
                )],
                None,
            ),
            LvFunc::new(
                "lv_style_init".to_string(),
                vec![LvArg::new(
                    "style".to_string(),
                    LvType::new("abc".to_string()),
                )],
                None,
            ),
        ];

        let widget_names = CodeGen::get_widget_names(&funcs);
        let widgets = CodeGen::extract_widgets(&funcs).unwrap();
        println!("{:?}", widgets.iter().map(|w| w.name.clone()).collect_vec());
        assert_eq!(widget_names.len(), 4);
    }

    #[test]
    fn generate_method_wrapper() {
        // pub fn lv_arc_set_bg_end_angle(arc: *mut lv_obj_t, end: u16);
        let arc_set_bg_end_angle = LvFunc::new(
            "lv_arc_set_bg_end_angle".to_string(),
            vec![
                LvArg::new("obj".to_string(), LvType::new("* mut lv_obj_t".to_string())),
                LvArg::new("end".to_string(), LvType::new("u16".to_string())),
            ],
            None,
        );
        let arc_widget = LvWidget {
            name: "arc".to_string(),
            methods: vec![],
        };

        let code = arc_set_bg_end_angle.code(&arc_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_arc_set_bg_end_angle(obj: &mut crate::widgets::Wdg, end: u16) {
                unsafe {
                    lightvgl_sys::lv_arc_set_bg_end_angle(obj.raw_mut(), end)
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_str_types_as_argument() {
        let bindgen_code = quote! {
            extern "C" {
                #[doc = " Set a new text for a label. Memory will be allocated to store the text by the label."]
                #[doc = " @param label pointer to a label object"]
                #[doc = " @param text '\\0' terminated character string. NULL to refresh with the current text."]
                pub fn lv_label_set_text(label: *mut lv_obj_t, text: *const c_char);
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let label_set_text = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "label".to_string(),
            methods: vec![],
        };

        let code = label_set_text.code(&parent_widget).unwrap();
        let expected_code = quote! {

            pub fn lv_label_set_text(label: &mut crate::widgets::Wdg, text: &CStr) {
                unsafe {
                    lightvgl_sys::lv_label_set_text(
                        label.raw_mut(),
                        text.as_ptr()
                    )
                }
            }

        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_mut_str_types_as_argument() {
        let bindgen_code = quote! {
            unsafe extern "C" {
                pub fn lv_roller_get_option_str(
                    obj: *const lv_obj_t,
                    option: u32,
                    buf: *mut c_char,
                    buf_size: u32,
                ) -> lv_result_t;
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let dropdown_get_selected_str = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "dropdown".to_string(),
            methods: vec![],
        };

        let code = dropdown_get_selected_str.code(&parent_widget).unwrap();
        let expected_code = quote! {

            pub fn lv_roller_get_option_str(
                obj: &crate::widgets::Wdg,
                option: u32,
                buf: &mut ::alloc::ffi::CString,
                buf_size: u32
            ) -> lv_result_t {
                unsafe {
                    let buf_raw = buf.clone().into_raw();
                    let rust_result = lightvgl_sys::lv_roller_get_option_str(obj.raw(), option, buf_raw, buf_size);
                    *buf = ::alloc::ffi::CString::from_raw(buf_raw);
                    rust_result
                }
            }

        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_with_mut_obj_parameter() {
        let bindgen_code = quote! {
            extern "C" {
                pub fn lv_arc_rotate_obj_to_angle(
                    obj: *const lv_obj_t,
                    obj_to_rotate: *mut lv_obj_t,
                    r_offset: lv_coord_t,
                );
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let arc_rotate_obj_to_angle = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "arc".to_string(),
            methods: vec![],
        };

        let code = arc_rotate_obj_to_angle.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_arc_rotate_obj_to_angle(obj: &crate::widgets::Wdg, obj_to_rotate: &mut crate::widgets::Wdg, r_offset: lv_coord_t) {
                unsafe {
                    lightvgl_sys::lv_arc_rotate_obj_to_angle(
                        obj.raw(),
                        obj_to_rotate.raw_mut(),
                        r_offset
                    )
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_void_return() {
        let bindgen_code = quote! {
            extern "C" {
                #[doc = " Set a new text for a label. Memory will be allocated to store the text by the label."]
                #[doc = " @param label pointer to a label object"]
                #[doc = " @param text '\\0' terminated character string. NULL to refresh with the current text."]
                pub fn lv_label_set_text(label: *mut lv_obj_t, text: *const c_char);
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let label_set_text = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "label".to_string(),
            methods: vec![],
        };

        let code = label_set_text.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_label_set_text(label: &mut crate::widgets::Wdg, text: &CStr) {
                unsafe {
                    lightvgl_sys::lv_label_set_text(
                        label.raw_mut(),
                        text.as_ptr()
                    )
                }
            }
        };
        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_boolean_return() {
        let bindgen_code = quote! {
            extern "C" {
                pub fn lv_label_get_recolor(label: *mut lv_obj_t) -> bool;
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let label_get_recolor = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "label".to_string(),
            methods: vec![],
        };

        let code = label_get_recolor.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_label_get_recolor(label: &mut crate::widgets::Wdg) -> bool {
                unsafe {
                    lightvgl_sys::lv_label_get_recolor(label.raw_mut())
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_uint32_return() {
        let bindgen_code = quote! {
            extern "C" {
                pub fn lv_label_get_text_selection_start(label: *mut lv_obj_t) -> u32;
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let label_get_text_selection_start = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "label".to_string(),
            methods: vec![],
        };

        let code = label_get_text_selection_start.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_label_get_text_selection_start(label: &mut crate::widgets::Wdg) -> u32 {
                unsafe {
                    lightvgl_sys::lv_label_get_text_selection_start(label.raw_mut())
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_obj_return() {
        let bindgen_code = quote! {
            unsafe extern "C" {
                pub fn lv_dropdown_get_list(obj: *mut lv_obj_t) -> *mut lv_obj_t;
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let dropdown_get_list = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "obj".to_string(),
            methods: vec![],
        };

        let code = dropdown_get_list.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_dropdown_get_list(obj: &mut crate::widgets::Wdg) -> Option<Wdg> {
                unsafe {
                    let pointer = lightvgl_sys::lv_dropdown_get_list(obj.raw_mut());
                    Wdg::try_from_ptr(pointer)
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_str_return() {
        let bindgen_code = quote! {
            unsafe extern "C" {
                pub fn lv_label_get_text(obj: *const lv_obj_t) -> *mut c_char;
            }
        };
        let cg = CodeGen::load_func_defs(bindgen_code.to_string().as_str()).unwrap();

        let label_get_text = cg.get(0).unwrap().clone();
        let parent_widget = LvWidget {
            name: "obj".to_string(),
            methods: vec![],
        };

        let code = label_get_text.code(&parent_widget).unwrap();
        let expected_code = quote! {
            pub fn lv_label_get_text(obj: &crate::widgets::Wdg) -> &CStr {
                unsafe {
                    let pointer = lightvgl_sys::lv_label_get_text(obj.raw());
                    CStr::from_ptr(pointer)
                }
            }
        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_basic_widget_code() {
        let arc_widget = LvWidget {
            name: "arc".to_string(),
            methods: vec![],
        };

        let code = arc_widget.code(&()).unwrap();
        let expected_code = quote! {};

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_widget_with_constructor_code() {
        // pub fn lv_arc_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
        let arc_create = LvFunc::new(
            "lv_arc_create".to_string(),
            vec![
                LvArg::new("par".to_string(), LvType::new("*mut lv_obj_t".to_string())),
                LvArg::new(
                    "copy".to_string(),
                    LvType::new("*const lv_obj_t".to_string()),
                ),
            ],
            Some(LvType::new("*mut lv_obj_t".to_string())),
        );

        let arc_widget = LvWidget {
            name: "arc".to_string(),
            methods: vec![arc_create],
        };

        let code = arc_widget.code(&()).unwrap();
        let expected_code = quote! {};

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    /// cargo test list_unimplemented_functions -- --nocapture --ignored
    #[test]
    #[ignore]
    fn list_unimplemented_functions() {
        use proc_macro2::TokenStream;
        let source = lightvgl_sys::_bindgen_raw_src();

        let codegen = CodeGen::from(source).unwrap();
        let _widgets_impl: Vec<TokenStream> = codegen
            .get_widgets()
            .iter()
            .flat_map(|w| w.code(&()))
            .collect();
    }
}
