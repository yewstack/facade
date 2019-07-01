use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["web", "build"])
        .current_dir("ui")
        .output()
        .unwrap();
    if !output.status.success() {
        let out = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", out);
        panic!("Can't compile");
    }
}
