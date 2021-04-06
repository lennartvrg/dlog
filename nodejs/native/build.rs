use std::env;

extern crate neon_build;

fn main() {
    let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    std::fs::create_dir(std::path::Path::new(&out_dir).join("build")).unwrap();

    neon_build::Setup::options().output_dir("build").setup();
}
