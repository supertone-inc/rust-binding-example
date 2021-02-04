extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_name = env::var("PROJECT_NAME").unwrap();
    let include_dir = format!("{}/include", project_name);
    let header_file_name = format!("{}.h", project_name);
    let header_file_path = format!("{}/{}", include_dir, header_file_name);

    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file(header_file_path);
}
