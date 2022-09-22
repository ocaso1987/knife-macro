use std::collections::HashMap;

use knife_util::{
    context::ContextTrait,
    template::{ContextType, TemplateContextExt},
    Value,
};

use crate::base::main::{InputInfo, MacroTrait};

#[derive(Default)]
pub(crate) struct AsValueDerive {}

impl MacroTrait for AsValueDerive {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_struct {
            panic!("不支持在该语法块上使用AsValue宏");
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
            impl {{crate_builtin_name}}::util::bean::AsValueTrait for {{struct_name}} {
                fn as_value(&self) -> {{crate_builtin_name}}::util::Result<{{@root.crate_builtin_name}}::util::Value> {
                    let mut map = std::collections::BTreeMap::<String, {{crate_builtin_name}}::util::Value>::new();
                    {{#each field_name_list}}
                    match {{@root.crate_builtin_name}}::util::bean::AsValueTrait::as_value(&self.{{this}}) {
                        Ok(v) => match v{
                            {{@root.crate_builtin_name}}::util::Value::Null => {},
                            _=> {
                                map.insert("{{this}}".to_string(), v);
                            }
                        },
                        Err(e) => return Err(e),
                    };
                    {{/each}}
                    {{crate_builtin_name}}::util::OK({{crate_builtin_name}}::util::Value::Object(map))
                }
            }
            "#,
            vec!["field_name_list","crate_dryrun","struct_name","crate_builtin_name"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        );
    }
}
