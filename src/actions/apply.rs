use std::{
    fs,
    path::{absolute, Path},
};

use color_eyre::Result;

use crate::{
    config::{self, GitRepo},
    git::Git,
};

pub fn apply() -> Result<()> {
    let config = config::read()?;
    let path = absolute(Path::new(&config.name))?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    let patches = absolute(Path::new("patches"))?;
    let git = Git(&path);

    checkout_from_upstream(&git, config.upstream, &config.r#ref)?;

    for file in &config.unneeded_files {
        let _ = std::fs::remove_dir_all(path.join(file));
        let _ = std::fs::remove_file(path.join(file));
    }

    git.exec(&["add", "."])?;
    git.exec(&[
        "commit",
        "-m",
        "Initial",
        "--author=Initial Source <auto@mated.null>",
        "--allow-empty",
    ])?;
    let _ = git.exec(&["tag", "-d", "base"]);
    git.exec(&["tag", "base"])?;

    apply_patches(&git, &patches)?;

    Ok(())
}

fn checkout_from_upstream(git: &Git, upstream: GitRepo, reference: &str) -> Result<()> {
    let _ = git.exec(&["init", "--quiet"]);
    git.disable_auto_gpg_signing()?;
    let _ = git.exec(&["remote", "remove", "upstream"]);
    git.exec(&["remote", "add", "upstream", &upstream.https_url().unwrap()])?;
    git.exec(&["fetch", "upstream", "--prune"])?;
    if git.exec(&["checkout", "main"]).is_err() {
        git.exec(&["checkout", "-b", "main"])?;
    }
    git.exec(&["reset", "--hard", reference])?;
    let _ = git.exec(&["gc"]);
    Ok(())
}

fn apply_patches(git: &Git, patch_dir: &Path) -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let mail_dir = tempdir.path().join("new");
    fs::create_dir_all(&mail_dir)?;

    for patch in patch_dir.read_dir()?.flatten() {
        if patch.file_name().to_str().unwrap().ends_with(".patch") {
            fs::copy(patch.path(), mail_dir.join(patch.file_name()))?;
        }
    }

    let _ = git.exec(&["am", "--abort"]);
    match git.exec(&[
        "am",
        "--3way",
        "--ignore-whitespace",
        absolute(tempdir.path())?.to_str().unwrap(),
    ]) {
        Ok(_) => {
            println!("Patches applied cleanly");
        }
        Err(_) => {
            println!("Failed to apply patches.");
            println!("Fix patches and rebuild with `patchouli rebuild`");
        }
    }
    Ok(())
}
