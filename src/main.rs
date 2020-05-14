use flate2::read::GzDecoder;
use tar::Archive;

use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

mod analyze;
pub use analyze::*;

mod error;
pub use error::*;

pub mod config;
use config::*;

pub mod github;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_file(&PathBuf::from("config.toml"))?;

    let tmp_dir = std::env::temp_dir().join("lang-char-freq");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let languages: Vec<&Language> = vec!["javascript", "typescript"].into_iter().map(|id| &config.languages[&String::from(id)]).collect();

    for language in languages {
        let search = github::search(&language.id).await?;

        let repos = search
            .repositories
            .iter()
            .filter(|repo| !language.exclude_repos.contains(&repo.full_name))
            .take(config.repo_count);

        // TODO skip existing repositories
        for repo in repos {
            println!("Repository: {}", repo.full_name);

            // download
            println!("  Downloading ({} bytes)...", repo.size);
            let archive = repo.download_archive(&tmp_dir).await?;

            // extract
            println!("  Extracting...");
            let archive = GzDecoder::new(File::open(archive)?);
            let mut archive = Archive::new(archive);
            match archive.unpack(&tmp_dir) {
                Ok(_) => {}
                Err(err) => match err.kind() {
                    ErrorKind::AlreadyExists => println!("    Archive already exists, skipping."),
                    _ => return Result::Err(Error::Io(err)),
                },
            }
        }

        println!("Analyzing all repositories");
        let results = analyze_files(&tmp_dir, "rust", &config);
        let mut results = results.unwrap().into_iter().collect::<Vec<(char, usize)>>();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("{:?}", results);
    }

    Ok(())
}
