use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::lazy::SyncOnceCell;

pub static LATEST_MANIFEST: SyncOnceCell<VersionManifest> = SyncOnceCell::new();

#[derive(Debug)]
pub struct VersionManifest {
    pub latest: String,
    pub versions: HashMap<String, NaiveDateTime>,
}
