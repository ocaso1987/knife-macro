//!　普通容器注册
use std::collections::HashMap;

use crate::base::{create_derive_attribute_from, InputInfo, MacroTrait};
use darling::{FromMeta, ToTokens};
use knife_util::{
    crates::bson::{bson, Bson},
    ContextExt, TemplateContext, TemplateContextExt, VecExt,
};

/// 过程宏定义参数
#[derive(FromMeta)]

pub(crate) struct KnifeComponentMacro {
    name: String,
    scope: Option<String>,
    init: Option<String>,
    async_init: Option<String>,
    generate_method: Option<String>,
    target_method: Option<String>,
    crate_dryrun: Option<bool>,
    crate_builtin_name: Option<String>,
}

impl MacroTrait for KnifeComponentMacro {
    fn config(&self, config: &mut HashMap<String, Bson>) {
        config.insert_bool("with_item_struct", true);
    }

    fn init(&self, _input: &mut InputInfo, _config: &mut HashMap<String, Bson>) {}

    fn load(
        &self,
        context: &mut TemplateContext,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Bson>,
    ) {
        if !input.is_item_struct {
            panic!("不支持在该语法块上使用knife_component注解");
        }
        let generate_method = self
            .generate_method
            .as_ref()
            .unwrap_or(&"default".to_string())
            .to_string();
        context.insert_string("name", self.name.to_string());
        context.insert_string(
            "scope",
            self.scope
                .as_ref()
                .unwrap_or(&"global".to_string())
                .to_string(),
        );
        context.insert_string("generate_method", generate_method.to_string());
        context.insert_string(
            "target_method",
            self.target_method
                .as_ref()
                .unwrap_or(&"get_instance".to_string())
                .to_string(),
        );
        context.insert_bool("crate_dryrun", self.crate_dryrun.unwrap_or(false));
        context.insert_string(
            "crate_builtin_name",
            self.crate_builtin_name
                .as_ref()
                .unwrap_or(&"knife_framework".to_string())
                .to_string(),
        );
        context.insert_string(
            "init",
            self.init.as_ref().unwrap_or(&"".to_string()).to_string(),
        );
        context.insert_string(
            "async_init",
            self.async_init
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        );

        if "default" == generate_method.to_string() {
            let struct_attrs =
                create_derive_attribute_from(&input.item_struct.as_ref().unwrap().attrs, "Default")
                    .map(|x| x.to_token_stream().to_string());
            context.insert_bson("origin_struct_attrs_quote", bson!(struct_attrs));
        } else {
            let struct_attrs = &input
                .item_struct
                .as_ref()
                .unwrap()
                .attrs
                .map(|x| x.to_token_stream().to_string());
            context.insert_bson("origin_struct_attrs_quote", bson!(struct_attrs));
        }
        context.insert_string(
            "origin_struct_quote",
            format!(
                "{} {} {} {}",
                input.item_struct.as_ref().unwrap().vis.to_token_stream(),
                input
                    .item_struct
                    .as_ref()
                    .unwrap()
                    .struct_token
                    .to_token_stream(),
                input.item_struct.as_ref().unwrap().ident.to_token_stream(),
                input.item_struct.as_ref().unwrap().fields.to_token_stream()
            ),
        );
        context.insert_string(
            "ident",
            input
                .item_struct
                .as_ref()
                .unwrap()
                .ident
                .to_token_stream()
                .to_string(),
        );
    }

    fn process(&self, context: &mut TemplateContext, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
                {{#each origin_struct_attrs_quote}}
                {{{this}}}
                {{/each}}
                {{{origin_struct_quote}}}
                impl {{ident}} {
                    pub fn {{target_method}}() -> &'static mut Self {
                        let holder = {{ident}}__HOLDER_INSTANCE.as_mut::<{{ident}}__Holder>();
                        holder.init();
                        &mut holder.target
                    }
                    pub async fn {{target_method}}_async() -> &'static mut Self {
                        let holder = {{ident}}__HOLDER_INSTANCE.as_mut::<{{ident}}__Holder>();
                        holder.init_async().await;
                        &mut holder.target
                    }
                }
                lazy_static::lazy_static! {
                    static ref {{ident}}__HOLDER_INSTANCE: {{crate_builtin_name}}::util::AnyRef = {
                        {{crate_builtin_name}}::util::AnyRef::new({{crate_builtin_name}}::get_{{scope}}::<{{ident}}__Holder>("component".to_string(),"{{name}}".to_string()).unwrap())
                    };
                }
                struct {{ident}}__Holder {
                    target: {{ident}},
                    inited: bool, // 缺把锁
                }
                impl {{ident}}__Holder {
                    fn init(&self) {
                        if !self.inited {
                            {{#if init}}self.target.{{init}}();{{/if}}
                        }
                    }
                    async fn init_async(&self) {
                        if !self.inited {
                            {{#if async_init}}self.target.{{async_init}}().await;{{/if}}
                        }
                    }
                }
                impl {{crate_builtin_name}}::ComponentTrait for {{ident}}__Holder {}
                impl Into<{{crate_builtin_name}}::Component> for {{ident}}__Holder {
                    fn into(self) -> {{crate_builtin_name}}::Component {
                        {{crate_builtin_name}}::Component::COMPONENT(Box::new(self))
                    }
                }
                #[{{crate_builtin_name}}::crates::ctor::ctor]
                fn {{ident}}__INIT() {
                    {{crate_builtin_name}}::register_{{scope}}("component".to_string(),"{{name}}".to_string(),{{ident}}__Holder {
                        target: {{ident}}::{{generate_method}}(),
                        inited: false,
                    });
                }
            "#,
            vec!["origin_struct_attrs_quote","origin_struct_quote","ident","scope","name","generate_method","target_method","init","async_init","crate_builtin_name"],
        );
    }
}
