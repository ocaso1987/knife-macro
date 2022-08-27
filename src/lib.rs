//! 通用宏定义
extern crate proc_macro;

pub(crate) mod base;
pub(crate) mod boot {
    pub(crate) mod server;
}
pub(crate) mod container {
    pub(crate) mod component;
    pub(crate) mod router;
}
// pub(crate) mod crates {
//     pub(crate) mod sqlx;
// }

use base::knife_main;
use boot::server::KnifeServerMacro;
use container::{component::KnifeComponentMacro, router::KnifeRouterMacro};
// use crates::sqlx::sql_main;
use proc_macro::TokenStream;

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
    knife_main::<KnifeComponentMacro>(attr, input)
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
    knife_main::<KnifeRouterMacro>(attr, input)
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
    knife_main::<KnifeServerMacro>(attr, input)
}

// /// SQL执行语句
// ///
// /// # Example
// /// sql!("select * from t_table where id = ?", id);
// /// ```
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     sql_main(input)
// }
