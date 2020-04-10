use bytes::buf::ext::BufExt;

use futures::StreamExt;

use serde::Deserialize;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub total_count: usize,
    pub incomplete_results: bool,
    #[serde(rename = "items")]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: usize,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub html_url: String,
    #[serde(rename = "url")]
    pub api_url: String,
    pub fork: bool,
    // TODO parse
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub homepage: Option<String>,
    pub size: usize,
    pub stargazers_count: usize,
    pub watchers_count: usize,
    pub forks_count: usize,
    pub open_issues_count: usize,
    pub license: Option<License>, // unsure whether this will ever be null/excluded
    pub default_branch: String,
}

impl Repository {
    pub fn archive_url(&self) -> String {
        println!("{}", self.api_url);
        format!("{}/tarball", self.api_url)
    }

    // TODO handle reqwest error as well
    pub async fn download_archive<P: AsRef<Path>>(&self, dir: &P) -> std::io::Result<PathBuf> {
        let client = reqwest::Client::new();
        let response = client
            .get(&self.archive_url())
            .header(reqwest::header::USER_AGENT, "reqwest")
            .send()
            .await.unwrap()
            .bytes()
            .await.unwrap();

        let path = dir.as_ref().join(format!(
            "{user}-{repo}.tar.gz",
            user = self.owner.login,
            repo = self.name
        ));

        let mut file = File::create(&path)?;
        file.write_all(&response)?;

        Ok(path)
    }
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
    pub id: usize,
    #[serde(rename = "url")]
    pub api_url: String,
    // pub type: OwnerType,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: Option<String>,
}

pub async fn search(language: &str) -> reqwest::Result<SearchResponse> {
    let url = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc",
        language
    );
    let client = reqwest::Client::new();
    client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "reqwest")
        .send()
        .await?
        .json::<SearchResponse>()
        .await
}
