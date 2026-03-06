mod builtin;
mod types;

use std::path::Path;

pub use types::{Layout, Level};

#[cfg(test)]
pub use builtin::builtin_colemak;

fn load_from_file(path: &Path) -> anyhow::Result<Layout> {
    let content = std::fs::read_to_string(path)?;
    let layout: Layout = toml::from_str(&content)?;
    Ok(layout)
}

fn load_from_directory(dir: &Path) -> Vec<Layout> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut layouts = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "toml") {
            match load_from_file(&path) {
                Ok(l) => layouts.push(l),
                Err(e) => tracing::warn!("Failed to load layout {}: {e}", path.display()),
            }
        }
    }
    layouts
}

impl Layout {
    pub fn discover_all(extra_paths: &[String]) -> Vec<Self> {
        let mut layouts = builtin::all();

        // XDG default dir (silent if missing)
        if let Some(dir) = crate::utils::config_dir().map(|d| d.join("keydrill/layouts")) {
            layouts.extend(load_from_directory(&dir));
        }

        // Config-specified paths
        for entry in extra_paths {
            let path = Path::new(entry);
            if !path.exists() {
                tracing::warn!("Layout path not found: {}", path.display());
                continue;
            }
            if path.is_dir() {
                layouts.extend(load_from_directory(path));
            } else {
                match load_from_file(path) {
                    Ok(l) => layouts.push(l),
                    Err(e) => tracing::warn!("Failed to load layout {}: {e}", path.display()),
                }
            }
        }

        layouts
    }
}
