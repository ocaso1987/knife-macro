use super::base::{InputInfo, MacroTrait};
use knife_util::{
    context::ContextExt,
    template::{render_template_recursion, ContextType},
    value::Value,
};
use std::{collections::HashMap, str::FromStr};

/// 通用Derive处理单元
pub(crate) fn knife_derive_main<T>(
    input: &mut InputInfo,
    config: &mut HashMap<String, Value>,
    context: &mut HashMap<String, ContextType>,
) -> proc_macro::TokenStream
where
    T: MacroTrait + Default,
{
    let macro_target: T = Default::default();
    macro_target.config(config);
    macro_target.init(input, config);
    macro_target.load(context, input, config);
    macro_target.process(context, input);

    let result_quote = render_template_recursion(&context, "result").unwrap().0;
    let crate_dryrun = context.get_bool("crate_dryrun");
    if crate_dryrun {
        println!("-------------------------------------------------------------------------");
        println!("{}", result_quote);
        println!("-------------------------------------------------------------------------");
        proc_macro::TokenStream::from_str("").unwrap()
    } else {
        proc_macro::TokenStream::from_str(&result_quote).unwrap()
    }
}
