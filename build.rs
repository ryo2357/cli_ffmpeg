use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let package_name = env::var("CARGO_PKG_NAME").expect("Failed to get package name");
    let package_description = env::var("CARGO_PKG_DESCRIPTION").expect("Failed to get package description");

    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("package_info.rs");

    let contents = format!(
      r#"pub const APP_NAME: &str = "{}";
pub const APP_DESCRIPTION: &str = "{}";"#,
      package_name, package_description
  );

  fs::write(&dest_path, contents).expect("Failed to write package_info.rs");
}


