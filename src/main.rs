use flate2::read::GzDecoder;
use tar::Archive;

use std::collections::BTreeMap;
use std::fs::File;

mod analyze;
pub use analyze::*;

mod error;
pub use error::*;

pub mod config;
use config::*;

pub mod github;

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = std::env::temp_dir().join("lang-char-freq");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let search = github::search("Rust").await?;

    for repo in search.repositories.iter().skip(2).take(4) {
        println!("downloading repo {}", repo.full_name);
        let archive = repo.download_archive(&tmp_dir).await?;
        println!("extracting");
        let archive = GzDecoder::new(File::open(archive)?);
        let mut archive = Archive::new(archive);
        archive.unpack(&tmp_dir)?;
    }

    println!("ANALYZING");
    let results = analyze_files(&tmp_dir, "rust", &Config::from_file(&std::path::PathBuf::from("config.toml")).unwrap());
    let mut results = results.unwrap().into_iter().collect::<Vec<(char, usize)>>();
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("{:#?}", results);

    Ok(())
}
