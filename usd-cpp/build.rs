use std::io::prelude::*;
use std::process::Command;

/*
python thirdparty/USD/build_scripts/build_usd.py
--build-monolithic
--no-tests
--no-examples
--no-tutorials
--no-tools
--no-docs
--no-python
--no-imaging
--no-ptex
--no-openvdb
--no-usdview
--no-embree
--no-prman
--no-openimageio
--no-opencolorio
--no-alembic
--no-hdf5
--no-draco
--no-materialx
./
*/

fn build_cpp_usd(out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // The script directory
    let mut script_dir = std::path::PathBuf::from(std::env::current_dir().unwrap());
    script_dir.push("thirdparty");
    script_dir.push("USD");
    script_dir.push("build_scripts");
    script_dir.push("build_usd.py");

    // The result directory
    let mut cpp_out_dir = std::path::PathBuf::new();
    cpp_out_dir.push(&out_dir);
    cpp_out_dir.push("install");

    // The lib directory
    let mut lib_dir = cpp_out_dir.clone();
    lib_dir.push("lib");

    // The include directory
    let mut include_dir = cpp_out_dir.clone();
    include_dir.push("include");

    /*
    lib_dir.push("build");
    lib_dir.push("USD");
    lib_dir.push("pxr");
    */

    println!("Downloading dependencies and building USD c++ library");
    // Run the command to build the python c++ library
    let result = Command::new("python")
        .arg(script_dir)
        .arg("--build-monolithic")
        .arg("--no-tests")
        .arg("--no-examples")
        .arg("--no-tutorials")
        .arg("--no-tools")
        .arg("--no-docs")
        .arg("--no-python")
        .arg("--no-imaging")
        .arg("--no-ptex")
        .arg("--no-openvdb")
        .arg("--no-usdview")
        .arg("--no-embree")
        .arg("--no-prman")
        .arg("--no-openimageio")
        .arg("--no-opencolorio")
        .arg("--no-alembic")
        .arg("--no-hdf5")
        .arg("--no-draco")
        .arg("--no-materialx")
        .arg(cpp_out_dir)
        .current_dir(out_dir)
        .status()
        .unwrap();

    assert!(result.success());

    let lib = std::path::PathBuf::from("usd_ms");

    [include_dir, lib_dir, lib]
}

fn write_lib_info(info: [std::path::PathBuf; 3]) {

    // Make sure the source directory exists
    let src_path = std::path::PathBuf::from("src");
    std::fs::create_dir(src_path)
    let mut lib_path = src_path.clone();
    lib_path.push("lib.rs");

    write!(
        std::fs::File::create(lib_path).unwrap(),
        "\
pub const INCLUDE : &str = \"{}\"; \n\
pub const LIBS : &str = \"{}\"; \n\
pub const LIB : &str = \"{}\"; \n\
",
        info[0].to_str().unwrap(),
        info[1].to_str().unwrap(),
        info[2].to_str().unwrap(),
    );
}

fn main() {
    // Only run this build job if the USD source directory has changed
    println!("cargo:rerun-if-changed=thirdparty/USD");

    // The out directory of the build
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Build the usd cpp library
    write_lib_info(build_cpp_usd(&out_dir));
}
