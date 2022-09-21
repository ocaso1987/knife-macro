//!　服务器方式启动
use std::collections::HashMap;

use darling::{FromMeta, ToTokens};
use knife_util::{
    bean::AsValueTrait,
    context::ContextTrait,
    crates_builtin::serde_json::json,
    iter::VecExt,
    template::{ContextType, TemplateContextExt},
    Value,
};

use crate::base::main::{InputInfo, MacroTrait};

/// 过程宏定义参数
#[derive(FromMeta)]
pub(crate) struct KnifeServerMacro {
    project: String,
    application: String,
    crate_dryrun: Option<bool>,
    crate_builtin_name: Option<String>,
}

impl MacroTrait for KnifeServerMacro {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_fn {
            panic!("不支持在该语法块上使用knife_router宏");
        }
        context
            .insert_string("project", self.project.to_string())
            .unwrap();
        context
            .insert_string("application", self.application.to_string())
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
        context
            .insert_string("boot_type", "server".to_string())
            .unwrap();
        context
            .insert_string(
                "block_quote",
                input
                    .item_fn
                    .as_ref()
                    .unwrap()
                    .block
                    .to_token_stream()
                    .to_string(),
            )
            .unwrap();
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
            vec!["env_vars", "boot_type", "block_quote", "crate_builtin_name"]
                .map_collect(|x| x.to_string()),
        );
        context.insert_invoker(
            "env_vars",
            Box::new(|ctx| -> Value {
                json!({
                    "knife_project_id":     ctx.get_string("project").unwrap(),
                    "knife_application_id": ctx.get_string("application").unwrap()
                })
                .as_value()
                .unwrap()
            }),
        );
    }
}
