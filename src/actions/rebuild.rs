use std::path::{absolute, Path};

use color_eyre::Result;

use crate::{config, git::Git};

pub fn rebuild() -> Result<()> {
    let config = config::read()?;
    let git = Git(&Path::new(&config.name));
    let patches_dir = absolute("patches")?;
    git.exec(&[
        "format-patch",
        "--zero-commit",
        "--full-index",
        "--no-signature",
        "--no-stat",
        "-N",
        "-o",
        patches_dir.to_str().unwrap(),
        "base",
    ])?;
    Ok(())
}
