use anyhow::Result;
use std::env;
use tokio::fs;

pub const WORKDIR_NAME: &str = ".nature";
pub const REQUIREMENTS_TXT: &str = "requirements.txt";

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    let workdir = cwd.join(WORKDIR_NAME);
    fs::create_dir_all(&workdir).await?;

    fs::hard_link(
        cwd.join(REQUIREMENTS_TXT),
        workdir.join(REQUIREMENTS_TXT),
        ).await?;

    Ok(())
}
