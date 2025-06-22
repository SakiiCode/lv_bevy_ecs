use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::{ToTokens, format_ident};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use syn::{FnArg, ForeignItem, ForeignItemFn, Item, ReturnType, TypePath, parse_str};

type CGResult<T> = Result<T, Box<dyn Error>>;

const LIB_PREFIX: &str = "lv_";

const FUNCTION_BLACKLIST: [&'static str; 7] = [
    "lv_obj_null_on_delete",
    "lv_obj_add_style",
    "lv_obj_replace_style",
    "lv_obj_remove_style",
    "lv_obj_remove_style_all",
    "lv_obj_refresh_style",
    "lv_style_init"
];

#[derive(Debug, Copy, Clone)]
pub enum WrapperError {
    Skip,
}

pub type WrapperResult<T> = Result<T, WrapperError>;

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
        let methods: Vec<TokenStream> = self.methods.iter().flat_map(|m| m.code(self)).collect();
        Ok(quote! {
            #(#methods)*
        })
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
        if new_name.as_str().eq("create") && parent.name != "obj"
            || FUNCTION_BLACKLIST.contains(&self.name.as_str())
        {
            return Err(WrapperError::Skip);
        }

        // Handle return values
        let return_type = match self.ret {
            // function returns void
            None => quote!(()),
            // function returns something
            _ => {
                let return_value: &LvType = self.ret.as_ref().unwrap();
                if !return_value.is_pointer() {
                    parse_str(&return_value.literal_name).expect(&format!(
                        "Cannot parse {} as type",
                        return_value.literal_name
                    ))
                } else {
                    println!("Return value is pointer ({})", return_value.literal_name);
                    return Err(WrapperError::Skip);
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
        let args_decl =
            self.args
                .iter()
                .fold(quote!(), |args_accumulator, arg| {
                    let next_arg = arg.code(self).unwrap();

                    // If the accummulator is empty then we call quote! only with the next_arg content
                    if args_accumulator.is_empty() {
                        quote! {#next_arg}
                    }
                    // Otherwise we append next_arg at the end of the accumulator
                    else {
                        quote! {#args_accumulator, #next_arg}
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
                quote! {#var.raw()}
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

        // NOTE: When the function returns something we can 'avoid' placing an Ok() at the end.
        let explicit_ok = if return_type.is_empty() {
            quote!(Ok(()))
        } else {
            quote!()
        };

        // Append a semicolon at the end of the unsafe code only if there's no return value.
        // Otherwise we should remove it
        let optional_semicolon = match self.ret {
            None => quote!(;),
            _ => quote!(),
        };
        Ok(quote! {
            pub fn #func_name(#args_decl) -> #return_type {
                unsafe {
                    #args_preprocessing
                    lvgl_sys::#func_name(#ffi_args)#optional_semicolon
                    #args_postprocessing
                    #explicit_ok
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
                *#name = cstr_core::CString::from_raw(#name_raw);
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
        }else if self.typ.is_const_style() || self.typ.is_mut_style() {
            quote! {
                #ident.raw()
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

    pub fn is_array(&self) -> bool {
        self.literal_name.starts_with("* mut *") || self.literal_name.starts_with("* const *")
    }
}

impl Rusty for LvType {
    type Parent = LvArg;

    fn code(&self, _parent: &Self::Parent) -> WrapperResult<TokenStream> {
        let val = if self.is_array() {
            println!("Array as argument ({})", self.literal_name);
            return Err(WrapperError::Skip);
        } else if self.is_const_str() {
            quote!(&cstr_core::CStr)
        } else if self.is_mut_str() {
            quote!(&mut cstr_core::CString)
        } else if self.is_const_native_object() {
            quote!(&crate::widgets::Widget)
        } else if self.is_mut_native_object() {
            quote!(&mut crate::widgets::Widget)
        } else if self.is_const_style() {
            quote!(&mut crate::styles::Style) // TODO make this const
        } else if self.is_mut_style() {
            quote!(&mut crate::styles::Style)
        } else {
            let literal_name = self.literal_name.as_str();
            let raw_name = literal_name.replace("* const ", "").replace("* mut ", "");
            if raw_name == "cty :: c_void" {
                println!("Void pointer as argument ({literal_name})");
                return Err(WrapperError::Skip);
            }
            let ty: TypePath =
                parse_str(&raw_name).expect(&format!("Cannot parse {raw_name} to a type"));
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
            pub fn lv_arc_set_bg_end_angle(obj: &mut crate::widgets::Widget, end: u16) -> () {
                unsafe {
                    lvgl_sys::lv_arc_set_bg_end_angle(obj.raw(), end);
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
                pub fn lv_label_set_text(label: *mut lv_obj_t, text: *const ::core::ffi::c_char);
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

            pub fn lv_label_set_text(label: &mut crate::widgets::Widget, text: &cstr_core::CStr) -> () {
                unsafe {
                    lvgl_sys::lv_label_set_text(
                        label.raw(),
                        text.as_ptr()
                    );
                }
            }

        };

        assert_eq!(code.to_string(), expected_code.to_string());
    }

    #[test]
    fn generate_method_wrapper_for_mut_str_types_as_argument() {
        let bindgen_code = quote! {
            extern "C" {
                pub fn lv_dropdown_get_selected_str(obj: *const lv_obj_t, buf: *mut ::core::ffi::c_char, buf_size: u32);
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

            pub fn lv_dropdown_get_selected_str(obj: &mut crate::widgets::Widget, buf: &mut cstr_core::CString, buf_size:u32) -> () {
                unsafe {
                    let buf_raw = buf.clone().into_raw();
                    lvgl_sys::lv_dropdown_get_selected_str(
                        obj.raw(),
                        buf_raw,
                        buf_size
                    );
                    *buf = cstr_core::CString::from_raw(buf_raw);
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
                pub fn lv_label_set_text(label: *mut lv_obj_t, text: *const ::core::ffi::c_char);
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
            pub fn lv_label_set_text(label: &mut crate::widgets::Widget, text: &cstr_core::CStr) -> () {
                unsafe {
                    lvgl_sys::lv_label_set_text(
                        label.raw(),
                        text.as_ptr()
                    );
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
            pub fn lv_arc_rotate_obj_to_angle(obj: &mut crate::widgets::Widget, obj_to_rotate: &mut crate::widgets::Widget, r_offset: lv_coord_t) -> () {
                unsafe {
                    lvgl_sys::lv_arc_rotate_obj_to_angle(
                        obj.raw(),
                        obj_to_rotate.raw(),
                        r_offset
                    );
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
            pub fn lv_label_get_recolor(label: &mut crate::widgets::Widget) -> bool {
                unsafe {
                    lvgl_sys::lv_label_get_recolor(
                        label.raw()
                    )
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
            pub fn lv_label_get_text_selection_start(label: &mut crate::widgets::Widget) -> u32 {
                unsafe {
                    lvgl_sys::lv_label_get_text_selection_start(
                        label.raw()
                    )
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
}
