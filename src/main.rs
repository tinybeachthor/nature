use anyhow::Result;
use std::env;
use tokio::{fs, process::Command};

pub const WORKDIR_NAME: &str = ".nature";
pub const REQUIREMENTS_TXT: &str = "requirements.txt";

pub const GITIGNORE: &str = include_str!("../templates/.gitignore");
pub const FLAKE:     &str = include_str!("../templates/flake.nix");
pub const PROJECTS:  &str = include_str!("../templates/projects.toml");
pub const PYPROJECT: &str = include_str!("../templates/pyproject.toml");
pub const SETUP:     &str = include_str!("../templates/setup.py");

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    let workdir = cwd.join(WORKDIR_NAME);

    fs::create_dir_all(&workdir).await?;

    fs::hard_link(
        cwd.join(REQUIREMENTS_TXT),
        workdir.join(REQUIREMENTS_TXT),
        ).await?;

    fs::write(workdir.join(".gitignore"), GITIGNORE).await?;
    fs::write(workdir.join("flake.nix"), FLAKE).await?;
    fs::write(workdir.join("projects.toml"), PROJECTS).await?;
    fs::write(workdir.join("pyproject.toml"), PYPROJECT).await?;
    fs::write(workdir.join("setup.py"), SETUP).await?;

    Command::new("git")
        .arg("init")
        .current_dir(&workdir)
        .spawn()?
        .wait()
        .await?;
    Command::new("git")
        .arg("add")
        .arg("--all")
        .current_dir(&workdir)
        .spawn()?
        .wait()
        .await?;

    Command::new("nix")
        .arg("run")
        .arg(".#resolveImpure")
        .current_dir(&workdir)
        .spawn()?
        .wait()
        .await?;

    println!(r#"
    Setup complete

    To use the shell run:
        nix shell .nature
    "#);

    Ok(())
}
