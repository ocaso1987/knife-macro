//! 服务定义
use std::collections::HashMap;

use darling::{FromMeta, ToTokens};
use knife_util::{
    context::ContextTrait,
    crates_builtin::serde_json::json,
    template::{ContextType, TemplateContextExt},
    Value,
};

use crate::base::main::{InputInfo, MacroTrait};

/// 过程宏定义参数
#[derive(FromMeta)]
pub(crate) struct FieldSpecMacro {
    target_type: Option<String>,
    target_name: Option<String>,
    config_type: Option<String>,
    config: String,
    output_wrap: Option<bool>,
    crate_dryrun: Option<bool>,
    crate_builtin_name: Option<String>,
}

impl MacroTrait for FieldSpecMacro {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_fn {
            panic!("不支持在该语法块上使用field_spec宏");
        }
        context
            .insert_string("scope", "global".to_string())
            .unwrap();
        context
            .insert_bool("crate_dryrun", self.crate_dryrun.unwrap_or(false))
            .unwrap();
        context
            .insert_string(
                "crate_builtin_name",
                self.crate_builtin_name
                    .as_ref()
                    .unwrap_or(&"::knife_framework".to_string())
                    .to_string(),
            )
            .unwrap();

        let fn_attrs = &input
            .item_fn
            .as_ref()
            .unwrap()
            .attrs
            .iter()
            .map(|x| x.to_token_stream().to_string())
            .collect::<Vec<String>>();
        context
            .insert_json("origin_fn_attrs_quote", &json!(fn_attrs))
            .unwrap();
        context
            .insert_string(
                "vis_sig_quote",
                format!(
                    "{} {}",
                    input.item_fn.as_ref().unwrap().vis.to_token_stream(),
                    input.item_fn.as_ref().unwrap().sig.to_token_stream(),
                ),
            )
            .unwrap();
        context
            .insert_string(
                "block_quote",
                format!(
                    "{}",
                    input.item_fn.as_ref().unwrap().block.to_token_stream()
                ),
            )
            .unwrap();
        context
            .insert_string(
                "ident",
                input
                    .item_fn
                    .as_ref()
                    .unwrap()
                    .sig
                    .ident
                    .to_token_stream()
                    .to_string(),
            )
            .unwrap();

        context
            .insert_string(
                "target_name",
                self.target_name
                    .as_ref()
                    .expect("参数target_name不能为空")
                    .to_string(),
            )
            .unwrap();
        context
            .insert_string("config", self.config.to_string())
            .unwrap();
    }

    fn process(&self, context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
            {{#each origin_fn_attrs_quote}}
            {{{this}}}
            {{/each}}
            #[doc="<pre>{{target_name}}:{{config}}</pre>"]
            {{{vis_sig_quote}}} {
                {{{block_quote}}}
            }
            "#,
            vec![
                "origin_fn_attrs_quote",
                "vis_sig_quote",
                "block_quote",
                "target_name",
                "config",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        );
    }

    fn config(&mut self, _config: &mut HashMap<String, Value>) {
        if self.target_type.is_none() {
            self.target_type.replace("output".to_string());
        }
        if self.config_type.is_none() {
            self.config_type.replace("csv".to_string());
        }
        if self.target_type.as_ref().unwrap() == "output" && self.target_name.is_none() {
            self.target_name.replace("_result".to_string());
        }
        if self.output_wrap.is_none() {
            self.output_wrap.replace(false);
        }
    }
}
