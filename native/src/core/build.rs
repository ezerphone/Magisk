use pb_rs::ConfigBuilder;
use pb_rs::types::FileDescriptor;

use crate::codegen::gen_cxx_binding;

#[path = "../include/codegen.rs"]
mod codegen;

#[allow(clippy::unwrap_used)]
fn main() {
    println!("cargo:rerun-if-changed=resetprop/proto/persistent_properties.proto");
    // Ezrim CI: the generated .rs lives in the source tree (not cached). If a restored build cache
    // marks this script as fresh but the file is missing, rerun so it is generated again.
    println!("cargo:rerun-if-changed=resetprop/proto/persistent_properties.rs");

    gen_cxx_binding("core-rs");

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
