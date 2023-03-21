use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use color_eyre::Result;

pub fn init() -> Result<()> {
    let config_path = Path::new(concat!(env!("CARGO_PKG_NAME", ".toml")));
    if !config_path.exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(include_bytes!("../../default-patchouli.toml"))?;
        println!("Created {}", config_path.to_string_lossy());
    }

    let patches_dir = Path::new("patches");
    if !patches_dir.exists() {
        fs::create_dir(patches_dir)?;
        println!("Created {}", patches_dir.to_string_lossy());
    }
    Ok(())
}
