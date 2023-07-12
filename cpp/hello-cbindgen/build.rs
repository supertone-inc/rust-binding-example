fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_file_path = "hello/include/hello/hello.h";

    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file(header_file_path);
}
