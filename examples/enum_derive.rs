use knife_macro::EnumName;

#[allow(dead_code)]
#[derive(EnumName)]
pub(crate) enum Type {
    String(String),
    Number(i32),
}

fn main() {}
