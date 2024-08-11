use std::path::Path;

fn main() {
    cynic_codegen::register_schema("hotcable")
    .from_sdl_file(Path::new(r".\schemas\hotcable.graphql"))
    .unwrap()
    .as_default()
    .unwrap();
}
