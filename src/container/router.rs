//! 路由注册
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
pub(crate) struct KnifeRouterMacro {
    path: String,
    method: Option<String>,
    crate_dryrun: Option<bool>,
    crate_builtin_name: Option<String>,
}

impl MacroTrait for KnifeRouterMacro {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_fn {
            panic!("不支持在该语法块上使用knife_router宏");
        }
        let method = self
            .method
            .as_ref()
            .unwrap_or(&"POST".to_string())
            .to_uppercase();
        context
            .insert_string("path", self.path.to_string())
            .unwrap();
        context.insert_string("method", method.to_string()).unwrap();
        context
            .insert_string("name", format!("{}:{}", method, self.path))
            .unwrap();
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
                "origin_fn_quote",
                format!(
                    "{} {} {}",
                    input.item_fn.as_ref().unwrap().vis.to_token_stream(),
                    input.item_fn.as_ref().unwrap().sig.to_token_stream(),
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
    }

    fn process(&self, context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
                {{#each origin_fn_attrs_quote}}
                {{{this}}}
                {{/each}}
                {{{origin_fn_quote}}}
                {{crate_builtin_name}}::crates::lazy_static::lazy_static! {
                    static ref {{ident}}__HOLDER_INSTANCE: {{crate_builtin_name}}::util::any::AnyRef = {
                        {{crate_builtin_name}}::util::any::AnyRef::new({{crate_builtin_name}}::get_{{scope}}::<{{ident}}__Holder>("router".to_string(),"{{name}}".to_string()).unwrap())
                    };
                }
                struct {{ident}}__Holder {}
                #[{{crate_builtin_name}}::crates::async_trait::async_trait]
                impl {{crate_builtin_name}}::RouterTrait for {{ident}}__Holder {
                    async fn router_handle(&self, req: {{crate_builtin_name}}::HyperRequest) -> {{crate_builtin_name}}::HyperResponse {
                        let hyper_req = {{crate_builtin_name}}::util::future::AsyncInto::async_into(req).await;
                        let resp = {{ident}}(hyper_req).await;
                        let hyper_resp = {{crate_builtin_name}}::util::future::AsyncFrom::async_from((resp, "{{name}}".to_string())).await;
                        hyper_resp
                    }
                }
                impl From<{{ident}}__Holder> for {{crate_builtin_name}}::Component {
                    fn from(v:{{ident}}__Holder) -> Self {
                        {{crate_builtin_name}}::Component::ROUTER(Box::new(v))
                    }
                }
                #[{{crate_builtin_name}}::crates::ctor::ctor]
                fn {{ident}}__INIT() {
                    {{crate_builtin_name}}::register_{{scope}}("router".to_string(),"{{name}}".to_string(),{{ident}}__Holder {});
                    ::tracing::trace!("注册到路由:{{name}}",);
                }
            "#,
            vec!["origin_fn_attrs_quote","origin_fn_quote","ident","scope","name","crate_builtin_name"].iter().map(|x|x.to_string()).collect::<Vec<String>>(),
        );
    }
}
