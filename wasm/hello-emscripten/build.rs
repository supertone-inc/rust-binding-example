fn main() {
    generate_bindings();
    copy_cpp_headers();
}

fn generate_bindings() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_file_path = "include/hello/hello.h";

    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file(header_file_path);
}

fn copy_cpp_headers() {
    use std::env;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = PathBuf::from(&crate_dir).join("../../cpp/hello-cbindgen/hello/include/hello");
    let dst_dir = PathBuf::from(&crate_dir).join("include/hello");

    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());

        if src_path.extension() == Some(OsStr::new("hpp")) {
            fs::copy(src_path, dst_path).unwrap();
        }
    }
}
