use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=SHORT_GIT_HASH={}", git_hash);

    println!("cargo:rerun-if-changed={}", "proto");
    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&["proto/service.proto"], &["proto"])
        .unwrap_or_else(|err| panic!("protobuf compilation failed: {}", err));
}
