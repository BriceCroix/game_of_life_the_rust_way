use copy_to_output::copy_to_output;
use std::env;

fn main() {
    // Copy directory "assets" to target directory, rerun if any file change in this directory.
    println!("cargo:rerun-if-changed=assets/*");
    copy_to_output("assets", &env::var("PROFILE").unwrap())
        .expect("Could not copy \"assets\" directory");
}
