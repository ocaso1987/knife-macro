//! 宏定义通用操作类
use std::{collections::HashMap, str::FromStr};

use darling::FromMeta;
use knife_util::{
    context::ContextTrait,
    template::{render_template_recursion, ContextType},
    Value,
};
use syn::{parse_macro_input::parse, AttributeArgs};

use super::main::{InputInfo, MacroTrait};

/// 通用逻辑处理单元
pub(crate) fn knife_attribute_main<T>(
    input: &mut InputInfo,
    config: &mut HashMap<String, Value>,
    context: &mut HashMap<String, ContextType>,
) -> proc_macro::TokenStream
where
    T: FromMeta + MacroTrait,
{
    let attr_args = match parse::<AttributeArgs>(
        proc_macro::TokenStream::from_str(input.attr_source.as_ref().unwrap()).unwrap(),
    ) {
        Ok(v) => v,
        Err(e) => panic!("宏定义参数错误:{}", e),
    };
    let mut macro_target: T = FromMeta::from_list(&attr_args).unwrap();
    let _ = input.attr_args.insert(attr_args);
    macro_target.config(config);
    macro_target.init(input, config);
    macro_target.load(context, input, config);
    macro_target.process(context, input);

    let result_quote = render_template_recursion(context, "result").unwrap().0;
    let crate_dryrun = context.get_bool("crate_dryrun").unwrap();
    if crate_dryrun {
        println!("-------------------------------------------------------------------------");
        println!("{}", result_quote);
        println!("-------------------------------------------------------------------------");
        proc_macro::TokenStream::from_str(input.input_source.as_ref().unwrap()).unwrap()
    } else {
        proc_macro::TokenStream::from_str(&result_quote).unwrap()
    }
}
