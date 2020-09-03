extern crate cbindgen;

use std::env;
use std::path::{Path};

fn main()
{
  let p = env::var("CARGO_MANIFEST_DIR").unwrap();
  let crate_dir = Path::new(&p);
  let header_file_name = "adaptive.h";
  println!("");
  println!("crate directory: {:?}", crate_dir);
  let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml").to_str().unwrap()).unwrap();
  let output_dir = crate_dir.join("export").join("header");
  let header_file_location = output_dir.join(header_file_name);
  cbindgen::Builder::new()
    .with_crate(crate_dir)
    .with_config(config)
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file(header_file_location.clone());
}