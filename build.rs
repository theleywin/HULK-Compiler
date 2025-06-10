extern crate lalrpop;

use std::process::Command;

fn main() {
    //Generate parser with LALRPOP
    lalrpop::process_root().unwrap();

    //Probe LLVM for host triple and data layout
    let triple = Command::new("llvm-config")
        .arg("--host-target")
        .output()
        .expect("Failed to run `llvm-config --host-target`; is llvm-config in your PATH?");
    let datalayout = Command::new("llvm-config")
        .arg("--data-layout")
        .output()
        .expect("Failed to run `llvm-config --data-layout`; is llvm-config in your PATH?");

    //Convert to UTF-8 and trim newlines
    let triple = String::from_utf8(triple.stdout)
        .expect("`llvm-config --host-target` produced invalid UTF-8")
        .trim()
        .to_string();
    let datalayout = String::from_utf8(datalayout.stdout)
        .expect("`llvm-config --data-layout` produced invalid UTF-8")
        .trim()
        .to_string();

    //Expose to Rust code via environment variables
    println!("cargo:rustc-env=LLVM_HOST_TRIPLE={}", triple);
    println!("cargo:rustc-env=LLVM_DATA_LAYOUT={}", datalayout);

    //Invalidate build if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");
}
