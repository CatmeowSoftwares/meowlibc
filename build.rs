use cbindgen::{self, Config};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::warning=meow");
    let dir = walkdir::WalkDir::new("src");
    for entry in dir.into_iter() {
        let entry = entry?;
        if entry.file_type().is_dir() {
            let header_name = entry.file_name();
            println!("cargo::warning={header_name:?}");
            let d = std::fs::read_dir(entry.path())?;
            let config = d
                .filter_map(|x| x.ok())
                .find(|x| x.file_name() == "cbindgen.toml");
            println!("cargo::warning={config:?}");
            if let Some(config) = config {
                let header_name = entry.path().to_string_lossy().replacen("src", "", 1);
                cbindgen::Builder::new()
                    .with_config(Config::from_file(config.path())?)
                    .with_src(format!("{}/mod.rs", entry.path().to_str().expect("failed")))
                    .generate()?
                    .write_to_file(format!("include/{}.h", header_name));
                println!("cargo::warning=this should work");
            }
        }
    }
    Ok(())
}
