use flate2::read::GzDecoder;
use tar::Archive;

use std::fs::File;

mod error;
pub use error::*;

pub mod github;

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = std::env::temp_dir().join("lang-char-freq");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let search = github::search("Rust").await?;

    for repo in search.repositories {
        let archive = repo.download_archive(&tmp_dir).await?;
        let archive = GzDecoder::new(File::open(archive)?);
        let mut archive = Archive::new(archive);
        archive.unpack(&tmp_dir)?;
    }

    Ok(())
}
