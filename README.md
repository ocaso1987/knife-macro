# knife-macro
Knife框架公共宏定义

[![Crates.io](https://img.shields.io/crates/v/knife-macro)](https://crates.io/crates/knife-macro)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ocaso1987/knife-macro/knife-macro)](https://github.com/ocaso1987/knife-macro)
[![docs.rs](https://img.shields.io/docsrs/knife-macro)](https://docs.rs/knife-macro)

## 说明
本项目用于为knife-framework框架提供预设的宏定义，这些宏定义大多无法脱离于knife-framewok框架外执行。

## 包含以下宏定义

* **knife_server:** 服务启动过程宏
```rust
#[knife_server(project = "knife", application = "knife-sample")]
fn main() {}
```

* **knife_component:** 全局容器过程宏
```rust
#[knife_component(name = "bean")]
pub struct Bean {}
```

* **knife_router:** 路由过程宏
```rust
#[knife_router(path="/")]
fn handler() -> &'static str {
    "Hello, world"
}
```