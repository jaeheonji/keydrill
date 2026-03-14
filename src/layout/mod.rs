mod builtin;
mod types;

use std::path::Path;

use anyhow::bail;

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

fn resolve_external_layouts(
    builtins: &[Layout],
    external: Vec<Layout>,
) -> anyhow::Result<Vec<Layout>> {
    let mut resolved = Vec::new();
    for mut layout in external {
        if layout.keys.is_empty() {
            if let Some(ref builtin_name) = layout.builtin {
                if let Some(b) = builtins
                    .iter()
                    .find(|b| b.name.eq_ignore_ascii_case(builtin_name))
                {
                    layout.keys = b.keys.clone();
                } else {
                    bail!(
                        "Layout '{}': builtin '{}' not found",
                        layout.name,
                        builtin_name
                    );
                }
            } else {
                bail!("Layout '{}': no keys and no builtin reference", layout.name);
            }
        } else if layout.builtin.is_some() {
            tracing::warn!(
                "Layout '{}': has explicit keys, ignoring builtin reference",
                layout.name
            );
        }
        resolved.push(layout);
    }
    Ok(resolved)
}

impl Layout {
    pub fn discover_all(extra_paths: &[String]) -> anyhow::Result<Vec<Self>> {
        let builtins = builtin::all();
        let mut external = Vec::new();

        // XDG default dir (silent if missing)
        if let Some(dir) = crate::utils::config_dir().map(|d| d.join("keydrill/layouts")) {
            external.extend(load_from_directory(&dir));
        }

        // Config-specified paths
        for entry in extra_paths {
            let path = Path::new(entry);
            if !path.exists() {
                tracing::warn!("Layout path not found: {}", path.display());
                continue;
            }
            if path.is_dir() {
                external.extend(load_from_directory(path));
            } else {
                match load_from_file(path) {
                    Ok(l) => external.push(l),
                    Err(e) => tracing::warn!("Failed to load layout {}: {e}", path.display()),
                }
            }
        }

        let external = resolve_external_layouts(&builtins, external)?;

        let mut layouts = builtins;
        layouts.extend(external);
        Ok(layouts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_external(toml_str: &str) -> Layout {
        toml::from_str(toml_str).unwrap()
    }

    #[test]
    fn resolve_builtin_copies_keys() {
        let builtins = super::builtin::all();
        let ext = parse_external(
            r#"
            name = "My Layout"
            builtin = "Colemak"
            [[levels]]
            name = "Home Row"
            new_keys = ["a", "r"]
            "#,
        );
        let resolved = resolve_external_layouts(&builtins, vec![ext]).unwrap();
        assert_eq!(resolved.len(), 1);
        assert!(!resolved[0].keys.is_empty());
    }

    #[test]
    fn resolve_builtin_case_insensitive() {
        let builtins = super::builtin::all();
        let ext = parse_external(
            r#"
            name = "My Layout"
            builtin = "colemak"
            [[levels]]
            name = "Home Row"
            new_keys = ["a", "r"]
            "#,
        );
        let resolved = resolve_external_layouts(&builtins, vec![ext]).unwrap();
        assert!(!resolved[0].keys.is_empty());
    }

    #[test]
    fn resolve_builtin_not_found() {
        let builtins = super::builtin::all();
        let ext = parse_external(
            r#"
            name = "My Layout"
            builtin = "Nonexistent"
            [[levels]]
            name = "Home Row"
            new_keys = ["a", "r"]
            "#,
        );
        let err = resolve_external_layouts(&builtins, vec![ext]).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn no_keys_no_builtin_errors() {
        let builtins = super::builtin::all();
        let ext = parse_external(
            r#"
            name = "My Layout"
            [[levels]]
            name = "Home Row"
            new_keys = ["a", "r"]
            "#,
        );
        let err = resolve_external_layouts(&builtins, vec![ext]).unwrap_err();
        assert!(err.to_string().contains("no keys and no builtin"));
    }

    #[test]
    fn explicit_keys_win_over_builtin() {
        let builtins = super::builtin::all();
        let ext = parse_external(
            r#"
            name = "My Layout"
            builtin = "Colemak"
            [[keys]]
            row = 1
            col = 0
            normal = "x"
            [[levels]]
            name = "Home Row"
            new_keys = ["x"]
            "#,
        );
        let resolved = resolve_external_layouts(&builtins, vec![ext]).unwrap();
        assert_eq!(resolved[0].keys.len(), 1);
        assert_eq!(resolved[0].keys[0].normal, 'x');
    }
}
