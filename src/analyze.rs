use crate::config::*;

use walkdir::WalkDir;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn analyze_files<P: AsRef<Path>>(
    dir: &P, lang: &str, config: &Config,
) -> crate::Result<BTreeMap<char, usize>> {
    let language = config.languages.get(lang).expect("could not find language in config");

    let mut total_freqs = BTreeMap::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let is_source = match path.extension() {
                None => false,
                Some(os_str) => match os_str.to_str() {
                    Some(s) => language.extensions.contains(&s.to_string()),
                    _ => false,
                },
            };

            if !is_source {
                continue;
            }

            // read
            if let Ok(contents) = fs::read_to_string(&path) {
                let freqs = analyze_contents(&contents, config);

                // add to total
                for (c, v) in freqs.iter() {
                    if (config.exclude_alphanumeric && c.is_alphanumeric()) || config.exclude_chars.contains(c) {
                        continue;
                    }
                    *total_freqs.entry(*c).or_default() += v;
                }
            } else {
                eprintln!("could not read file {:?}", path);
            }

        }
    }

    Ok(total_freqs)
}

fn analyze_contents(text: &str, config: &Config) -> BTreeMap<char, usize> {
    let mut freqs = BTreeMap::new();
    for c in text.chars() {
        if !config.exclude_chars.contains(&c)
            && (config.exclude_alphanumeric || !c.is_alphanumeric())
        {
            *freqs.entry(c).or_default() += 1;
        }
    }
    freqs
}
