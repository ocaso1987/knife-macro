//! 宏定义通用操作类
use std::{collections::HashMap, str::FromStr};

use darling::{FromMeta, ToTokens};
use knife_util::{render_template_recursion, serde_json::Value, ContextType, MapExt, StringExt};
use quote::{format_ident, quote};
use syn::{parse::Parser, parse_macro_input::parse, Attribute, AttributeArgs, ItemFn, ItemStruct};

/// 宏参数处理对象
pub(crate) trait MacroTrait {
    /// 设置初始化参数选项
    fn config(&self, _config: &mut HashMap<String, Value>) {}

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

#[derive(Default)]
pub(crate) struct InputInfo {
    /// 宏源代码
    pub(crate) attr_source: Option<String>,
    /// 目标代码
    pub(crate) item_source: Option<String>,
    /// 宏参数
    pub(crate) attr_args: Option<AttributeArgs>,
    /// 结构体
    pub(crate) item_struct: Option<ItemStruct>,
    /// 目标是否结构体
    pub(crate) is_item_struct: bool,
    /// 函数
    pub(crate) item_fn: Option<ItemFn>,
    /// 目标是否函数
    pub(crate) is_item_fn: bool,
}

/// 设置derive宏并增加指定参数
pub(crate) fn create_derive_attribute_from(
    attrs: &Vec<Attribute>,
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
    let is_match = attr_str.match_pattern(format!(r#"[^a-zA-Z0-9]{}"[^a-zA-Z0-9]"#, derive_arg));
    if !is_match {
        attr_str = attr_str.replace(
            "(",
            ("(".to_owned() + &derive_arg.to_string() + &", ".to_string()).as_str(),
        );
    }
    proc_macro2::TokenStream::from_str(attr_str.as_str()).unwrap()
}

/// 通用逻辑处理单元
pub(crate) fn knife_main<T>(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream
where
    T: FromMeta + MacroTrait,
{
    let input_info = &mut InputInfo::default();
    let _ = input_info.item_source.insert((&input).to_string());
    let _ = input_info.attr_source.insert((&attr).to_string());
    let attr_args = match parse::<AttributeArgs>(
        proc_macro::TokenStream::from_str(input_info.attr_source.as_ref().unwrap()).unwrap(),
    ) {
        Ok(v) => v,
        Err(e) => panic!("宏定义参数错误:{}", e),
    };
    let macro_target: T = FromMeta::from_list(&attr_args).unwrap();
    let _ = input_info.attr_args.insert(attr_args);
    let config = &mut HashMap::new();
    let context = &mut HashMap::new();
    macro_target.config(config);
    config_default(input_info, config);
    macro_target.init(input_info, config);
    init_default(input_info, config);
    macro_target.load(context, input_info, config);
    load_default(context, input_info, config);
    macro_target.process(context, input_info);
    process_default(context, input_info);
    let result_quote = render_template_recursion(&context, "result").unwrap();
    let crate_dryrun = context.get_bool("crate_dryrun");
    if crate_dryrun {
        println!("-------------------------------------------------------------------------");
        println!("{}", result_quote);
        println!("-------------------------------------------------------------------------");
        proc_macro::TokenStream::from_str(input_info.item_source.as_ref().unwrap()).unwrap()
    } else {
        proc_macro::TokenStream::from_str(&result_quote).unwrap()
    }
}

fn process_default(_context: &mut HashMap<String, ContextType>, _input_info: &mut InputInfo) {}

fn load_default(
    _context: &mut HashMap<String, ContextType>,
    _input_info: &mut InputInfo,
    _config: &mut HashMap<String, Value>,
) {
}

fn init_default(_input_info: &mut InputInfo, _config: &mut HashMap<String, Value>) {}

fn config_default(input_info: &mut InputInfo, config: &mut HashMap<String, Value>) {
    let with_item_fn = config.get_bool_or("with_item_fn", false);
    if with_item_fn {
        if let Ok(v) = parse::<ItemFn>(
            proc_macro::TokenStream::from_str(&input_info.item_source.as_ref().unwrap()).unwrap(),
        ) {
            let _ = input_info.item_fn.insert(v);
            input_info.is_item_fn = true;
        }
    }
    let with_item_struct = config.get_bool_or("with_item_struct", false);
    if with_item_struct {
        if let Ok(v) = parse::<ItemStruct>(
            proc_macro::TokenStream::from_str(&input_info.item_source.as_ref().unwrap()).unwrap(),
        ) {
            let _ = input_info.item_struct.insert(v);
            input_info.is_item_struct = true;
        }
    }
}
