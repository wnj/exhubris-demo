use std::path::PathBuf;

fn main() {
    let outpath = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    println!("cargo::rerun-if-changed=../pong/pong-idl.kdl");
    let iface = hubris_build::idl::load_interface("../pong/pong-idl.kdl").unwrap();
    let client = hubris_build::idl::codegen::generate_client(&iface).unwrap();
    let client = hubris_build::idl::codegen::format_code(&client);
    let genclient_path = outpath.join("generated_client.rs");

    std::fs::write(&genclient_path, client).unwrap();
}
