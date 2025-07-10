// src/resolver.rs

use serde::{Deserialize};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct TorchSource {
    pub cuda: String,
    pub source: String,
    pub url: String,
}


// Parses a version string like "12.3" into a tuple (major, minor)
fn parse_version(v: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = v.trim().split('.').collect();
    if parts.len() == 2 {
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        Some((major, minor))
    } else {
        None
    }
}

pub fn load_sources(json_path: &str) -> Vec<TorchSource> {
    let json = fs::read_to_string(json_path)
        .expect("Failed to read cuda_torch_sources.json");
    serde_json::from_str(&json)
        .expect("Failed to parse cuda_torch_sources.json")
}

pub fn resolve_best_source(detected_version: &str, sources: &[TorchSource]) -> TorchSource {
    let target = parse_version(detected_version);

    let mut best: Option<TorchSource> = None;

    for src in sources {
        if let Some(v) = parse_version(&src.cuda) {
            if let Some(target_v) = target {
                if v <= target_v {
                    match &best {
                        Some(best_src) if parse_version(&best_src.cuda).unwrap() >= v => continue,
                        _ => best = Some(src.clone()),
                    }
                }
            }
        }
    }

    best.unwrap_or_else(|| TorchSource {
        cuda: "cpu".to_string(),
        source: "pypi".to_string(),
        url: "https://pypi.org/simple".to_string(),
    })
}
