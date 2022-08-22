use std::{path::PathBuf, process::Command};

#[test]
fn bootstrap() {
    let iface_files = &["proto/health.proto"];
    let dirs = &["proto"];

    let out_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("generated");
    let grpc_health_v1_descriptor_set_path: PathBuf = out_dir.join("grpc_health_v1.bin");

    tonic_build::configure()
        .file_descriptor_set_path(grpc_health_v1_descriptor_set_path)
        .build_client(true)
        .build_server(true)
        .out_dir(format!("{}", out_dir.display()))
        .compile(iface_files, dirs)
        .unwrap();

    let status = Command::new("git")
        .arg("diff")
        .arg("--exit-code")
        .arg("--")
        .arg(format!("{}", out_dir.display()))
        .status()
        .unwrap();

    if !status.success() {
        panic!("You should commit the protobuf files");
    }
}
