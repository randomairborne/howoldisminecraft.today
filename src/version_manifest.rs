use crate::parsed_manifest::{VersionManifest as ParsedVersionManifest, LATEST_MANIFEST};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: Type,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Type {
    #[serde(rename = "old_alpha")]
    OldAlpha,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
}

pub fn update_manifest() -> reqwest::Result<()> {
    let VersionManifest { latest, versions } =
        reqwest::blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?
            .json::<VersionManifest>()?;

    let mut version_map = HashMap::with_capacity(versions.len());
    for version in versions {
        let parsed_time = match chrono::DateTime::parse_from_rfc3339(&*version.release_time) {
            Ok(t) => t.naive_utc(),
            Err(e) => {
                println!(
                    "warning: failed to parse timestamp as rfc3339 format: {}",
                    e
                );
                continue;
            }
        };
        if version_map.insert(version.id, parsed_time).is_some() {
            println!("warning: found duplicate keys in JSON response");
        };
    }

    let converted_manifest = ParsedVersionManifest {
        latest: latest.release,
        versions: version_map,
    };

    let _ = LATEST_MANIFEST.set(converted_manifest);

    Ok(())
}
