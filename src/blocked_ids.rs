use hbb_common::log;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

const RELOAD_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct BlockedIds {
    path: Arc<PathBuf>,
    state: Arc<Mutex<State>>,
}

struct State {
    ids: HashSet<String>,
    modified: Option<SystemTime>,
    last_check: Instant,
}

impl BlockedIds {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Arc::new(path.into());
        let (ids, modified) = load(&path);
        log::info!("Loaded {} blocked ID(s) from {}", ids.len(), path.display());
        Self {
            path,
            state: Arc::new(Mutex::new(State {
                ids,
                modified,
                last_check: Instant::now(),
            })),
        }
    }

    pub fn contains(&self, id: &str) -> bool {
        let normalized = normalize(id);
        if normalized.is_empty() {
            return false;
        }

        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(err) => {
                log::error!("Blocked ID state lock was poisoned: {}", err);
                return false;
            }
        };
        if state.last_check.elapsed() >= RELOAD_INTERVAL {
            state.last_check = Instant::now();
            let modified = fs::metadata(self.path.as_ref())
                .and_then(|metadata| metadata.modified())
                .ok();
            if modified != state.modified {
                let (ids, modified) = load(self.path.as_ref());
                log::info!(
                    "Reloaded {} blocked ID(s) from {}",
                    ids.len(),
                    self.path.display()
                );
                state.ids = ids;
                state.modified = modified;
            }
        }
        state.ids.contains(&normalized)
    }
}

fn load(path: &Path) -> (HashSet<String>, Option<SystemTime>) {
    let modified = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok();
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => {
            log::error!("Could not read blocked ID file {}: {}", path.display(), err);
            String::new()
        }
    };
    (parse(&contents), modified)
}

fn parse(contents: &str) -> HashSet<String> {
    contents
        .lines()
        .filter_map(|line| {
            let value = line.split('#').next().unwrap_or_default();
            let normalized = normalize(value);
            if normalized.is_empty() {
                None
            } else if normalized.len() < 6 || !normalized.bytes().all(|b| b.is_ascii_digit()) {
                log::warn!("Ignoring invalid blocked ID entry: {:?}", value.trim());
                None
            } else {
                Some(normalized)
            }
        })
        .collect()
}

fn normalize(id: &str) -> String {
    id.chars()
        .filter(|c| !c.is_ascii_whitespace() && *c != '-')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comments_and_formatted_ids() {
        let ids = parse("248 136 051 # test device\n123-456-789\ninvalid\n");
        assert!(ids.contains("248136051"));
        assert!(ids.contains("123456789"));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn missing_file_starts_with_empty_blocklist() {
        let path = std::env::temp_dir().join(format!(
            "inforia-blocked-ids-missing-{}",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);
        let blocked = BlockedIds::new(path);
        assert!(!blocked.contains("248136051"));
    }
}
