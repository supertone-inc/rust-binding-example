fn main() -> std::io::Result<()> {
  let source = format!("{}/libhello.dylib", env!("CRATE_OUT_DIR"));
  let target = format!("{}/hello/hello.so", env!("CRATE_MANIFEST_DIR"));

  std::fs::copy(source, target)?;

  Ok(())
}
