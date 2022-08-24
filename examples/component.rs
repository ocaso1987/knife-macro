use knife_macro::{knife_component, knife_server};

#[knife_component(name = "bean", crate_dryrun = true, async_init = "init")]
#[derive(Clone, Copy)]
#[doc = "docs"]
struct Bean {}

#[knife_server(project = "knife", application = "knife-sample", crate_dryrun = true)]
fn main() {}
