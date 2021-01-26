use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let pwd_dir = env::var_os("PWD").unwrap();
    let mut tmp_path = PathBuf::new();
    tmp_path.push(&pwd_dir);
    tmp_path.pop();
    tmp_path.push("templar/src/templar.pest");
    let tmp_path = tmp_path.as_path();

    let vlk_path = Path::new(&pwd_dir).join("src/valkyrie.pest");

    println!("cargo:rerun-if-changed={:?}", vlk_path);

    let tmp_content = fs::read_to_string(tmp_path).unwrap();
    let vlk_content = fs::read_to_string(vlk_path).unwrap();
    let gen_content = format!("{}\n{}", vlk_content, tmp_content);

    let out_path = Path::new(&pwd_dir).join("src/gen.pest");
    fs::write(
        &out_path,
        &gen_content,
    )
    .unwrap();
}
