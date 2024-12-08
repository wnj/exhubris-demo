use std::path::PathBuf;

fn main() {
    let outpath = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    println!("cargo::rerun-if-changed=pong-idl.kdl");
    let iface = hubris_build::idl::load_interface("pong-idl.kdl").unwrap();
    let server = hubris_build::idl::codegen::generate_server(&iface).unwrap();
    let server = hubris_build::idl::codegen::format_code(&server);
    let genserver_path = outpath.join("generated_server.rs");

    std::fs::write(&genserver_path, server).unwrap();
}
