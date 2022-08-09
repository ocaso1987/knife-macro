use knife_macro::{knife_router, knife_server};

#[knife_server(project = "knife", application = "knife-sample", crate_dryrun = true)]
fn main() {}

#[knife_router(path = "hello_world", crate_dryrun = true)]
async fn handler() -> String {
    "Hello, world".to_string()
}
