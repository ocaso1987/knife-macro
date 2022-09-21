use std::collections::HashMap;

use knife_util::{
    context::ContextTrait,
    iter::VecExt,
    template::{ContextType, TemplateContextExt},
    Value,
};

use crate::base::main::{InputInfo, MacroTrait};

#[derive(Default)]
pub(crate) struct MergeValueDerive {}

impl MacroTrait for MergeValueDerive {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_struct {
            panic!("不支持在该语法块上使用MergeValue宏");
        }
        context.insert_bool("crate_dryrun", false).unwrap();
        context
            .insert_string("crate_builtin_name", "::knife_framework".to_string())
            .unwrap();

        let field_name_list = input
            .item_struct
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .map(|x| Value::String(x.ident.as_ref().unwrap().to_string()))
            .collect::<Vec<Value>>();
        context
            .insert_string(
                "struct_name",
                input.item_struct.as_ref().unwrap().ident.to_string(),
            )
            .unwrap();
        context
            .insert_value("field_name_list", Value::Array(field_name_list))
            .unwrap();
    }

    fn process(&self, context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
            impl {{crate_builtin_name}}::util::bean::MergeValueTrait for {{struct_name}} {
                fn merge_value(&mut self, target: Option<&{{crate_builtin_name}}::util::Value>) -> {{crate_builtin_name}}::util::Result<Self> {
                    {{#each field_name_list}}
                    if let Some(v) = target {
                        self.{{this}} = {{@root.crate_builtin_name}}::util::bean::MergeValueTrait::merge_value(&mut self.{{this}}, {{@root.crate_builtin_name}}::util::context::ContextTrait::get_value(v, "{{this}}").unwrap()).unwrap();
                    }
                    {{/each}}
                    Ok(self.clone())
                }
            }
            "#,
            vec!["field_name_list","crate_dryrun","struct_name","crate_builtin_name"].map_collect(|x| x.to_string()),
        );
    }
}
