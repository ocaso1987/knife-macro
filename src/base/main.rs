use darling::ToTokens;
use knife_util::Value;
use knife_util::{template::ContextType, types::StringExt};
use quote::{format_ident, quote};
use std::{collections::HashMap, str::FromStr};
use syn::ItemEnum;
use syn::{parse::Parser, Attribute, AttributeArgs, ItemFn, ItemStruct};

#[derive(Default)]
pub(crate) struct InputInfo {
    /// 宏源代码
    pub(crate) attr_source: Option<String>,
    /// 宏参数
    pub(crate) attr_args: Option<AttributeArgs>,
    /// 目标代码
    pub(crate) input_source: Option<String>,
    /// 结构体
    pub(crate) item_struct: Option<ItemStruct>,
    /// 目标是否结构体
    pub(crate) is_item_struct: bool,
    /// 函数
    pub(crate) item_fn: Option<ItemFn>,
    /// 目标是否函数
    pub(crate) is_item_fn: bool,
    /// 枚举
    pub(crate) item_enum: Option<ItemEnum>,
    /// 目标是否函数
    pub(crate) is_item_enum: bool,
}

/// 宏参数处理对象
pub(crate) trait MacroTrait {
    /// 设置初始化参数选项
    fn config(&mut self, _config: &mut HashMap<String, Value>) {}

    /// 配置需要从代码中提取的内容
    fn init(&self, _input: &mut InputInfo, _config: &mut HashMap<String, Value>) {}

    /// 初始化上下文并存入默认参数
    fn load(
        &self,
        _context: &mut HashMap<String, ContextType>,
        _input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
    }

    /// 往上下文中存入默认参数、解析模板与计算逻辑
    fn process(&self, _context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {}
}

/// 设置derive宏并增加指定参数
pub(crate) fn create_derive_attribute_from(
    attrs: &[Attribute],
    derive_arg: &str,
) -> Vec<Attribute> {
    let mut attr_quotes = vec![];
    let derive_attr = attrs.iter().find(|x| x.path.is_ident("derive"));
    for attr in attrs.iter() {
        if attr.path.is_ident("derive") {
            attr_quotes.push(get_attr_quote(attr, derive_arg))
        } else {
            attr_quotes.push(quote! {#attr})
        }
    }
    if derive_attr.is_none() {
        let derive_arg_indent = format_ident!("{}", derive_arg);
        attr_quotes.push(quote! {#[derive(#derive_arg_indent)]})
    }
    let result: proc_macro2::TokenStream = attr_quotes
        .iter()
        .fold(quote! {}, |acc, new| quote! {#acc #new});
    Attribute::parse_outer
        .parse(proc_macro::TokenStream::from(result))
        .unwrap()
}

/// 设置derive宏并增加指定参数
fn get_attr_quote(attr: &Attribute, derive_arg: &str) -> proc_macro2::TokenStream {
    let mut attr_str = attr.to_token_stream().to_string();
    let is_match = attr_str.regex_match(format!(r#"[^a-zA-Z0-9]{}"[^a-zA-Z0-9]"#, derive_arg));
    if !is_match {
        attr_str = attr_str.replace('(', ("(".to_owned() + derive_arg + ", ").as_str());
    }
    proc_macro2::TokenStream::from_str(attr_str.as_str()).unwrap()
}
