use std::collections::HashMap;

use darling::ToTokens;
use knife_util::{
    bean::AsValueTrait,
    context::ContextTrait,
    crates_builtin::serde_json::json,
    template::{ContextType, TemplateContextExt},
    Value,
};

use crate::base::main::{InputInfo, MacroTrait};

#[derive(Default)]
pub(crate) struct EnumNameDerive {}

impl MacroTrait for EnumNameDerive {
    fn load(
        &self,
        context: &mut HashMap<String, ContextType>,
        input: &mut InputInfo,
        _config: &mut HashMap<String, Value>,
    ) {
        if !input.is_item_enum {
            panic!("不支持在该语法块上使用EnumName宏");
        }
        context.insert_bool("crate_dryrun", false).unwrap();
        let enum_name = input.item_enum.as_ref().unwrap().ident.to_string();
        context
            .insert_string("enum_name", enum_name.to_string())
            .unwrap();
        context
            .insert_string(
                "generics_quote",
                input
                    .item_enum
                    .as_ref()
                    .unwrap()
                    .generics
                    .to_token_stream()
                    .to_string(),
            )
            .unwrap();
        let mut enum_variant_array = Vec::<Value>::new();
        for (i, entry) in input
            .item_enum
            .as_ref()
            .unwrap()
            .variants
            .iter()
            .enumerate()
        {
            let name = entry.ident.to_string();
            let field_len = entry.fields.len();
            let key = if field_len > 0 {
                let place_str = "_,".repeat(field_len).trim_end_matches(',').to_string();
                format!("{}::{}({})", enum_name, name, place_str)
            } else {
                format!("{}::{}", enum_name, name,)
            };
            enum_variant_array.push(
                json!({
                    "index": i,
                    "key": key,
                    "name": name
                })
                .as_value()
                .unwrap(),
            );
        }
        context
            .insert_value("enum_variants", Value::Array(enum_variant_array))
            .unwrap();
    }

    fn process(&self, context: &mut HashMap<String, ContextType>, _input: &mut InputInfo) {
        context.insert_template(
            "result",
            r#"
                impl{{{generics_quote}}} {{enum_name}}{{{generics_quote}}} {
                    pub fn enum_name(&self) -> &'static str{
                        match self {
                            {{#each enum_variants}}
                            {{this.key}} => "{{this.name}}",
                            {{/each}}
                        }
                    }
                    pub fn enum_name_iter() -> Vec<&'static str>{
                        let mut arr = vec![];
                        {{#each enum_variants}}
                        arr.insert({{this.index}}, "{{this.name}}");
                        {{/each}}
                        arr
                    }
                }
            "#,
            vec!["enum_name", "generics_quote", "enum_variants"]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }
}
