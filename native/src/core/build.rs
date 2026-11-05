use pb_rs::ConfigBuilder;
use pb_rs::types::FileDescriptor;

use crate::codegen::gen_cxx_binding;

#[path = "../include/codegen.rs"]
mod codegen;

#[allow(clippy::unwrap_used)]
fn main() {
    println!("cargo:rerun-if-changed=resetprop/proto/persistent_properties.proto");

    gen_cxx_binding("core-rs");

    // Ezrim CI: the generated files are committed. Generating them here in the source tree raced
    // between the parallel per-ABI builds (one build read a half-written file). Only generate if missing.
    if std::path::Path::new("resetprop/proto/persistent_properties.rs").exists() {
        return;
    }

    let cb = ConfigBuilder::new(
        &["resetprop/proto/persistent_properties.proto"],
        None,
        Some(&"resetprop/proto"),
        &["."],
    )
    .unwrap();
    FileDescriptor::run(
        &cb.single_module(true)
            .dont_use_cow(true)
            .generate_getters(true)
            .build(),
    )
    .unwrap();
}
