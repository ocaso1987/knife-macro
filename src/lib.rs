//! 通用宏定义
extern crate proc_macro;

pub(crate) mod base {
    pub(crate) mod attribute;
    pub(crate) mod base;
    pub(crate) mod derive;
}
pub(crate) mod boot {
    pub(crate) mod server;
}
pub(crate) mod container {
    pub(crate) mod component;
    pub(crate) mod router;
}

pub(crate) mod enums {
    pub(crate) mod enum_name;
}

use base::{attribute::knife_attribute_main, base::InputInfo, derive::knife_derive_main};
use boot::server::KnifeServerMacro;
use container::{component::KnifeComponentMacro, router::KnifeRouterMacro};
use enums::enum_name::EnumNameDerive;
use proc_macro::TokenStream;
use std::collections::HashMap;
use syn::{ItemEnum, ItemFn, ItemStruct};

/// 全局容器过程宏
///
/// 类似于Spring中的Component注解，用于设置一个对象到上下文中
///
/// ## 支持参数
/// * name: 名称，必填
/// * scope: 作用域，默认为global
///
/// # Example
/// ```ignore
/// #[knife_component(name = "bean")]
/// pub struct Bean {}
/// ```
#[proc_macro_attribute]
pub fn knife_component(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_info = &mut InputInfo::default();
    let _ = input_info.input_source.insert((&input).to_string());
    let _ = input_info.attr_source.insert((&attr).to_string());
    let item_struct = syn::parse_macro_input::parse::<ItemStruct>(input).unwrap();
    let _ = input_info.item_struct.insert(item_struct);
    input_info.is_item_struct = true;

    knife_attribute_main::<KnifeComponentMacro>(
        input_info,
        &mut HashMap::new(),
        &mut HashMap::new(),
    )
}

/// 路由过程宏
///
/// ## 支持参数
/// * path: 路由路径，必填
/// * method: 请求方法，默认POST
///
/// # Example
/// ```ignore
/// #[router(path="/")]
/// fn handler() -> &'static str {
///     "Hello, world"
/// }
/// ```
#[proc_macro_attribute]
pub fn knife_router(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_info = &mut InputInfo::default();
    let _ = input_info.input_source.insert((&input).to_string());
    let _ = input_info.attr_source.insert((&attr).to_string());
    let item_fn = syn::parse_macro_input::parse::<ItemFn>(input).unwrap();
    let _ = input_info.item_fn.insert(item_fn);
    input_info.is_item_fn = true;

    knife_attribute_main::<KnifeRouterMacro>(input_info, &mut HashMap::new(), &mut HashMap::new())
}

/// 服务启动过程宏
///
/// 类似于Spring中的SpringBoot注解，标注代码从此处启动
///
/// ## 支持参数
/// * project: 项目组名称，必填
/// * application: 应用名称，必填
///
/// # Example
/// ```ignore
/// #[knife_server(project = "knife", application = "knife-sample")]
/// fn main() {}
/// ```
#[proc_macro_attribute]
pub fn knife_server(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_info = &mut InputInfo::default();
    let _ = input_info.input_source.insert((&input).to_string());
    let _ = input_info.attr_source.insert((&attr).to_string());
    let item_fn = syn::parse_macro_input::parse::<ItemFn>(input).unwrap();
    let _ = input_info.item_fn.insert(item_fn);
    input_info.is_item_fn = true;

    knife_attribute_main::<KnifeServerMacro>(input_info, &mut HashMap::new(), &mut HashMap::new())
}

/// 获取枚举名称，为枚举生成enum_name方法
#[proc_macro_derive(EnumName, attributes(knife_option))]
pub fn derive_enum_name(input: TokenStream) -> TokenStream {
    let input_info = &mut InputInfo::default();
    let _ = input_info.input_source.insert((&input).to_string());
    let item_enum = syn::parse_macro_input::parse::<ItemEnum>(input).unwrap();
    let _ = input_info.item_enum.insert(item_enum);
    input_info.is_item_enum = true;

    knife_derive_main::<EnumNameDerive>(input_info, &mut HashMap::new(), &mut HashMap::new())
}
