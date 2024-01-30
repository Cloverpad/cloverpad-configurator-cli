use std::io::Result;

fn main() -> Result<()> {
    // Rebuild if protobuf defintions have changed
    // The .options files are specific to nanopb, so no need to rebuild if they change
    println!("cargo:rerun-if-changed=src/protocol/commands-config.proto");
    println!("cargo:rerun-if-changed=src/protocol/commands-main.proto");
    println!("cargo:rerun-if-changed=src/protocol/commands-state.proto");
    println!("cargo:rerun-if-changed=src/protocol/commands.proto");
    println!("cargo:rerun-if-changed=src/protocol/config.proto");
    println!("cargo:rerun-if-changed=src/protocol/state.proto");

    prost_build::compile_protos(&["src/protocol/commands.proto"], &["src/protocol"])?;
    Ok(())
}
