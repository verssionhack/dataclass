#![allow(unused)]
use proc_macro::TokenStream;
use std::str::FromStr;

use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, spanned::Spanned, Data, DeriveInput, Field, Fields, LitBool, LitStr,
    Meta,
};

struct Dataclass;

impl Dataclass {
    const META_PATH: &'static str = "dataclass";
}
struct DataclassTemplate;

impl DataclassTemplate {
    const RWLOCK: &'static str = "RwLock";
    const MUTEX: &'static str = "Mutex";
    const CELL: &'static str = "Cell";
    const REF_CELL: &'static str = "RefCell";

    const SYNC_RWLOCK_SETTER: &'static str = "*self.{}.write().unwrap() = {};";
    const SYNC_RWLOCK_GETTER: &'static str = "*self.{}.read().unwrap()";
    const SYNC_MUTEX_SETTER: &'static str = "*self.{}.lock().unwrap() = {};";
    const SYNC_MUTEX_GETTER: &'static str = "*self.{}.lock().unwrap()";
    const CELL_SETTER: &'static str = "self.{}.set({});";
    const CELL_GETTER: &'static str = "self.{}.get()";
    const REF_CELL_SETTER: &'static str = "*self.{}.borrow_mut() = {};";
    const REF_CELL_GETTER: &'static str = "*self.{}.borrow()";

    const U8: &'static str = "u8";
    const U16: &'static str = "u16";
    const U32: &'static str = "u32";
    const U64: &'static str = "u64";
    const U128: &'static str = "u128";

    const I8: &'static str = "i8";
    const I16: &'static str = "i16";
    const I32: &'static str = "i32";
    const I64: &'static str = "i64";
    const I128: &'static str = "i128";

    const F32: &'static str = "f32";
    const F64: &'static str = "f64";
    const STRING: &'static str = "String";
    const STR: &'static str = "str";

    const COPYABLE_TYPES: &'static [&'static str] = &[
        Self::U8,
        Self::U16,
        Self::U32,
        Self::U64,
        Self::U128,
        Self::I8,
        Self::I16,
        Self::I32,
        Self::I64,
        Self::I128,
        Self::F32,
        Self::F64,
        Self::STR,
    ];

    fn is_copyable(name: &str) -> bool {
        Self::COPYABLE_TYPES.contains(&name)
    }

    fn return_type(name: &str) -> Option<&'static str> {
        Some(match name {
            Self::U8 => Self::U8,
            Self::U16 => Self::U16,
            Self::U32 => Self::U32,
            Self::U64 => Self::U64,
            Self::U128 => Self::U128,
            Self::I8 => Self::I8,
            Self::I16 => Self::I16,
            Self::I32 => Self::I32,
            Self::I64 => Self::I64,
            Self::I128 => Self::I128,
            Self::F32 => Self::F32,
            Self::F64 => Self::F64,
            Self::STRING => Self::STR,
            _ => return None,
        })
    }

    fn input_type(name: &str) -> Option<&'static str> {
        Some(match name {
            Self::U8 => Self::U8,
            Self::U16 => Self::U16,
            Self::U32 => Self::U32,
            Self::U64 => Self::U64,
            Self::U128 => Self::U128,
            Self::I8 => Self::I8,
            Self::I16 => Self::I16,
            Self::I32 => Self::I32,
            Self::I64 => Self::I64,
            Self::I128 => Self::I128,
            Self::F32 => Self::F32,
            Self::F64 => Self::F64,
            Self::STRING => Self::STR,
            _ => return None,
        })
    }

    fn sync_rwlock_setter(name: &str, value: &str) -> String {
        format!("*self.{}.write().unwrap() = {};", name, value)
    }
    fn sync_rwlock_getter(name: &str) -> String {
        format!("*self.{}.read().unwrap()", name)
    }
    fn sync_mutex_setter(name: &str, value: &str) -> String {
        format!("*self.{}.lock().unwrap() = {};", name, value)
    }
    fn sync_mutex_getter(name: &str) -> String {
        format!("*self.{}.lock().unwrap()", name)
    }
    fn cell_setter(name: &str, value: &str) -> String {
        format!("self.{}.set({});", name, value)
    }
    fn cell_getter(name: &str) -> String {
        format!("self.{}.get()", name)
    }
    fn refcell_setter(name: &str, value: &str) -> String {
        format!("*self.{}.borrow_mut() = {};", name, value)
    }
    fn refcell_getter(name: &str) -> String {
        format!("*self.{}.borrow()", name)
    }

    fn match_setter_template(field: &Field, index: Option<usize>) -> Option<String> {
        let name = field
            .ident
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| index.unwrap().to_string());
        let ty = field
            .ty
            .span()
            .source_text()
            .to_token_stream()
            .to_string()
            .replace("\"", "");
        //println!("{:?}", field.ty.to_token_stream());
        Some(match ty.as_str() {
            Self::RWLOCK => Self::sync_rwlock_setter(&name, "value"),
            Self::MUTEX => Self::sync_mutex_setter(&name, "value"),
            Self::CELL => Self::cell_setter(&name, "value"),
            Self::REF_CELL => Self::refcell_setter(&name, "value"),
            _ => return None,
        })
    }

    fn match_getter_template(field: &Field, index: Option<usize>) -> Option<String> {
        let name = field
            .ident
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| index.unwrap().to_string());
        let ty = field
            .ty
            .span()
            .source_text()
            .to_token_stream()
            .to_string()
            .replace("\"", "");
        //println!("{:?}", field.ty.to_token_stream());
        //println!("match_type: {} len={}", &ty, ty.len());
        Some(match ty.as_str() {
            Self::RWLOCK => Self::sync_rwlock_getter(&name),
            Self::MUTEX => Self::sync_mutex_getter(&name),
            Self::CELL => Self::cell_getter(&name),
            Self::REF_CELL => Self::refcell_getter(&name),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum PubScope {
    Crate,
    Super,
    Self_,
}

impl ToString for PubScope {
    fn to_string(&self) -> String {
        match self {
            Self::Crate => "crate",
            Self::Super => "super",
            Self::Self_ => "self",
        }
        .to_string()
    }
}

impl FromStr for PubScope {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "crate" => Self::Crate,
            "super" => Self::Super,
            "self" => Self::Self_,
            _ => {
                panic!("Unsupport pub range: {}", s);
            }
        })
    }
}

#[derive(Debug, Default)]
struct DataclassAttribute {
    name: Option<String>,
    is_setter: bool,
    is_getter: bool,
    is_pub: bool,
    as_ref: bool,
    is_const: bool,
    is_skip: bool,
    pub_range: Option<PubScope>,
}

impl DataclassAttribute {
    const NAME: &'static str = "name";
    const SETTER: &'static str = "setter";
    const GETTER: &'static str = "getter";
    const ASREF: &'static str = "as_ref";
    const SKIP: &'static str = "skip";
    const PUB: &'static str = "pub";
    const PUB_SCOPE: &'static str = "pub_scope";
    const CONST: &'static str = "const";
}

impl DataclassAttribute {
    fn setter() -> Self {
        Self {
            is_setter: true,
            is_pub: true,
            ..Default::default()
        }
    }

    fn getter() -> Self {
        Self {
            is_getter: true,
            is_pub: true,
            as_ref: true,
            ..Default::default()
        }
    }

    fn setup(&mut self, meta: ParseNestedMeta<'_>) {
        //println!("setup meta: {}", meta.path.get_ident().unwrap());
        if meta.path.is_ident(Self::NAME) {
            self.name = meta
                .value()
                .map(|v| v.parse::<LitStr>().expect("name parse error").value())
                .ok();
            //println!("meta name: {:?}", self.name);
        } else if meta.path.is_ident(Self::ASREF) {
            self.as_ref = meta
                .value()
                .map(|v| v.parse::<LitBool>())
                .map(|v| v.map(|v| v.value()).ok() == Some(true))
                .unwrap_or(true);
        } else if meta.path.is_ident(Self::PUB) {
            self.is_pub = meta
                .value()
                .map(|v| v.parse::<LitBool>())
                .map(|v| v.map(|v| v.value()).ok() == Some(true))
                .unwrap_or(true);
        } else if meta.path.is_ident(Self::CONST) && self.is_getter {
            self.is_const = meta
                .value()
                .map(|v| v.parse::<LitBool>())
                .map(|v| v.map(|v| v.value()).ok() == Some(true))
                .unwrap_or(true);
        } else if meta.path.is_ident(Self::SKIP) {
            self.is_skip = true;
        } else if meta.path.is_ident(Self::PUB_SCOPE) {
            self.is_pub = true;
            self.pub_range = meta
                .value()
                .map(|v| v.parse::<LitStr>().expect("pub range parse error"))
                .map(|v| PubScope::from_str(&v.value()).expect("pub range parse error"))
                .ok();
        } else {
            //compile_error!("Unsupport attribute");
        }
    }

    fn of(meta: &Meta) -> (Self, Self) {
        let mut getter = DataclassAttribute::getter();
        let mut setter = DataclassAttribute::setter();
        let mut skip = false;
        match meta {
            Meta::Path(_path) => {}
            Meta::List(list) => {
                let _ = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident(Self::SKIP) {
                        getter.is_skip = true;
                        setter.is_skip = true;
                        skip = true;
                    }
                    if skip {
                        return Ok(());
                    }
                    //println!("meta: {}", meta.path.get_ident().map(|v| v.to_string()).expect("meta has no ident"));
                    if meta.path.is_ident(Self::SETTER) {
                        let _ = meta.parse_nested_meta(|meta| {
                            let _ = setter.setup(meta);
                            Ok(())
                        });
                    } else if meta.path.is_ident(Self::GETTER) {
                        let _ = meta.parse_nested_meta(|meta| {
                            let _ = getter.setup(meta);
                            Ok(())
                        });
                    }
                    Ok(())
                });
            }
            Meta::NameValue(_namavalue) => {}
        }
        (getter, setter)
    }

    fn generate_method(
        &mut self,
        method_name: Option<String>,
        field: &Field,
        index: Option<usize>,
    ) -> proc_macro2::TokenStream {
        let mut impl_str = String::new();
        let tp = field.ty.clone();
        let ty = field
            .ty
            .span()
            .source_text()
            .to_token_stream()
            .to_string()
            .replace("\"", "");
        if self.is_pub {
            impl_str += "pub"
        }
        if let Some(pub_range) = self.pub_range {
            impl_str += "(";
            impl_str += &pub_range.to_string();
            impl_str += ")";
        }
        impl_str += " ";
        if self.is_const && self.is_getter && DataclassTemplate::is_copyable(&ty) {
            impl_str += "const "
        }
        impl_str += "fn ";
        if let Some(name) = method_name {
            impl_str += &name;
        } else {
            impl_str += &self.name.take().expect("no name setup");
        }
        impl_str += "(&";
        if self.is_setter {
            if let Some(template) = DataclassTemplate::match_setter_template(field, index) {
                impl_str += "self, value: ";
                let tokens = tp.into_token_stream().into_iter().skip(2);
                let tp = field.ty.clone();
                let tokens_ = tp.into_token_stream().into_iter().skip(2);
                let len = tokens.count() - 1;
                impl_str += &tokens_
                    .take(len)
                    .collect::<proc_macro2::TokenStream>()
                    .to_string();
                impl_str += ") {";
                impl_str += &template;
            } else {
                impl_str += "mut self, value: ";
                impl_str += &field.ty.to_token_stream().to_string();
                impl_str += ") {";
                impl_str += &format!(
                    "self.{} = value;",
                    field
                        .ident
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| { index.expect("no index").to_string() })
                );
            }
        } else {
            impl_str += "self) -> ";
            if let Some(template) = DataclassTemplate::match_getter_template(field, index) {
                let tokens = tp.into_token_stream().into_iter().skip(2);
                let tp = field.ty.clone();
                let tokens_ = tp.into_token_stream().into_iter().skip(2);
                let len = tokens.count() - 1;
                impl_str += &tokens_
                    .take(len)
                    .collect::<proc_macro2::TokenStream>()
                    .to_string();
                //println!("template: {}", &template);
                //println!("impl_str: {}", &impl_str);
                impl_str += " {";
                impl_str += &template;
                //println!("impl_str_templated: {}\n", &impl_str);
            } else {
                if self.as_ref && !DataclassTemplate::is_copyable(&ty) {
                    impl_str += "&";
                }
                if let Some(ret_type) = DataclassTemplate::return_type(&ty) {
                    impl_str += ret_type;
                } else {
                    impl_str += &tp.to_token_stream().to_string();
                }
                impl_str += " {";
                if self.as_ref && !DataclassTemplate::is_copyable(&ty) {
                    impl_str += "&";
                }
                impl_str += &format!(
                    "self.{}",
                    field
                        .ident
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| { index.expect("no index").to_string() })
                );
            }
        }
        impl_str += "}";
        //println!("generate_method: {}\n", &impl_str);
        syn::parse_str(&impl_str).expect("method parse error")
    }
}

fn dataclass_named_field_impl(field: &Field) -> proc_macro2::TokenStream {
    let (mut getter, mut setter) = (DataclassAttribute::getter(), DataclassAttribute::setter());
    for attr in &field.attrs {
        if !attr.path().is_ident(Dataclass::META_PATH) {
            continue;
        }
        //println!("for named field: {:?}", field.ident);
        (getter, setter) = DataclassAttribute::of(&attr.meta);
        break;
    }
    let field_name = field.ident.as_ref().expect("field has no name");
    let field_name_str = {
        let mut _name = format!("{}", field_name);
        while _name.chars().nth(0) == Some('_') {
            _name = _name[1..].to_string();
        }
        while _name.chars().last() == Some('_') {
            _name = _name[.._name.len() - 1].to_string();
        }
        _name
    };
    let mut impls = Vec::new();

    if !getter.is_skip {
        impls.push(if getter.name.is_none() {
            getter.generate_method(Some(format!("get_{}", &field_name_str)), field, None)
        } else {
            getter.generate_method(None, field, None)
        });
    }

    if !setter.is_skip {
        impls.push(if setter.name.is_none() {
            setter.generate_method(Some(format!("set_{}", &field_name_str)), field, None)
        } else {
            setter.generate_method(None, field, None)
        });
    }


    //let _ty = &field.ty;
    //println!("Field type: {}",ty.span().source_text().to_token_stream().to_string());
    quote! {
        #(#impls)*
    }
}

fn dataclass_unnamed_field_impl(index: usize, field: &Field) -> proc_macro2::TokenStream {
    let (mut getter, mut setter) = (DataclassAttribute::getter(), DataclassAttribute::setter());
    for attr in &field.attrs {
        if !attr.path().is_ident(Dataclass::META_PATH) {
            continue;
        }
        //println!("for unnamed field: {}", index);
        (getter, setter) = DataclassAttribute::of(&attr.meta);
    }
    let field_name_str = format!("field_{}", index);
    let mut impls = Vec::new();

    //println!("getter: {:?}\n", getter);
    //println!("setter: {:?}\n", setter);

    if !getter.is_skip {
        impls.push(if getter.name.is_none() {
            getter.generate_method(Some(format!("get_{}", &field_name_str)), field, Some(index))
        } else {
            getter.generate_method(None, field, Some(index))
        });
    }

    if !setter.is_skip {
        impls.push(if setter.name.is_none() {
            setter.generate_method(Some(format!("set_{}", &field_name_str)), field, Some(index))
        } else {
            setter.generate_method(None, field, Some(index))
        });
    }

    //let _ty = &field.ty;
    //println!("Field type: {}",ty.span().source_text().to_token_stream().to_string());
    quote! {
        #(#impls)*
    }
}

#[proc_macro_derive(Dataclass, attributes(dataclass))]
pub fn dataclass_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &input.ident;

    let methods = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_methods = fields
                    .named
                    .iter()
                    .map(|field| dataclass_named_field_impl(field));
                quote! {
                    impl #struct_name {
                        #(#field_methods)*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_methods = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| dataclass_unnamed_field_impl(i, field));

                quote! {
                    impl #struct_name {
                        #(#field_methods)*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    impl #struct_name {
                        // No fields to generate methods for
                    }
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            quote! {
                compile_error!("Field methods not implemented for enums or unions.");
            }
        }
    };

    //let filename = format!("{}.template", struct_name.to_string());
    //std::fs::write(&filename,methods.to_string());
    TokenStream::from(methods)
}
