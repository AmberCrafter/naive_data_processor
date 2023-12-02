use std::{error::Error, path::PathBuf, env};

fn main() -> Result<(), Box<dyn Error>> {
    let proto_file = "./proto/qc.proto";
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        // .file_descriptor_set_path(out_dir.join("qc_description.bin"))
        .out_dir("./src/lib")
        .compile(&[proto_file], &["proto"])?;

    Ok(())
}