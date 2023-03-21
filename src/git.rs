use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

use color_eyre::{eyre::eyre, Result};

pub struct Git<'g>(pub &'g Path);

impl Git<'_> {
    const BASE_ARGS: &'static [&'static str] =
        &["-c", "commit.gpgsign=false", "-c", "core.safecrlf=false"];

    pub fn exec(&self, args: &[&str]) -> Result<()> {
        println!("GIT: git {}", args.join(" "));
        let output = Command::new("git")
            .args(Self::BASE_ARGS)
            .args(args)
            .current_dir(&self.0)
            .output()?;
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        if output.status.success() {
            Ok(())
        } else {
            Err(eyre!("Proccess failed"))
        }
    }

    pub fn disable_auto_gpg_signing(&self) -> Result<()> {
        self.exec(&["config", "commit.gpgSign", "false"])?;
        self.exec(&["config", "tag.gpgSign", "false"])
    }
}
