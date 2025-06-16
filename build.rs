use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let dest_path = Path::new("src").join("consts.rs");
    let consts = Command::new("python")
        .arg("buildconsts.py")
        .output()
        .unwrap();
    fs::write(&dest_path, consts.stdout.as_slice()).unwrap();
    println!("cargo::rerun-if-changed=build.rs,buildconsts.py");
}
