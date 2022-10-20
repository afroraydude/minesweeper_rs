use std::env;
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_path = get_output_path();

    let assets_path = Path::new(&manifest_dir_string).join("assets");
    let assets_output_path = output_path.join("assets");
    // create the output directory if it doesn't exist
    std::fs::create_dir_all(&assets_output_path).unwrap();

    // for every file in assets folder
    for entry in assets_path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let output_file_path = assets_output_path.join(file_name);
        println!("Transforming {} to {}", path.display(), output_file_path.display());
        // copy file to target folder
        std::fs::copy(path, output_file_path).unwrap();
    }
}