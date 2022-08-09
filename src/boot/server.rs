//!　服务器方式启动
use crate::base::{InputInfo, MacroTrait};
use darling::{FromMeta, ToTokens};
use knife_util::{
    serde_json::{json, Value},
    ContextMapUtil, ContextType, MapUtil,
};
use std::collections::HashMap;

/// 过程宏定义参数
#[derive(FromMeta)]
pub(crate) struct KnifeServerMacro {
    project: String,
    application: String,
    crate_dryrun: Option<bool>,
    crate_builtin_name: Option<String>,
}

impl MacroTrait for KnifeServerMacro {
    fn config(&self, config: &mut HashMap<String, Value>) {
        config.insert_bool("with_item_fn", true);
    }

    fn init(&self, _input: &mut InputInfo, _config: &mut HashMap<String, Value>) {}

    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_fn {
            panic!("不支持在该语法块上使用knife_router注解");
        }
        context.insert_string("project", self.project.to_string());
        context.insert_string("application", self.application.to_string());
        context.insert_bool("crate_dryrun", self.crate_dryrun.unwrap_or(false));
        context.insert_string(
            "crate_builtin_name",
            self.crate_builtin_name
                .as_ref()
                .unwrap_or(&"knife_framework".to_string())
                .to_string(),
        );
        context.insert_string("boot_type", "server".to_string());
        context.insert_string(
            "block_quote",
            input
                .item_fn
                .as_ref()
                .unwrap()
                .block
                .to_token_stream()
                .to_string(),
        );
    }

    fn process(&self, context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
                pub fn main() {
                    {{#each env_vars}}
                    std::env::set_var("{{@key}}", "{{this}}");
                    {{/each}}
                    {{crate_builtin_name}}::start_{{boot_type}}(|| {
                        {{{block_quote}}}
                    });
                }
            "#,
            vec!["env_vars", "crate_builtin_name", "boot_type", "block_quote"],
        );
        context.insert_invoker(
            "env_vars",
            Box::new(|ctx| -> Value {
                json!({
                    "knife_project_id":     ctx.get_string("project"),
                    "knife_application_id": ctx.get_string("application")
                })
            }),
        );
    }
}
