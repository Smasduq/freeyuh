//! Desktop applications indexing and launching service.
//!
//! Scans XDG application directories, parses `.desktop` files, and provides
//! fast search and execution for the Spotlight app launcher.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub icon: String,
    pub exec: String,
    pub terminal: bool,
    pub keywords: Vec<String>,
}

/// Standard directories where `.desktop` files are stored.
fn application_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let home_path = Path::new(&home);
        dirs.push(home_path.join(".local/share/applications"));
        dirs.push(home_path.join(".local/share/flatpak/exports/share/applications"));
    }

    if let Ok(data_dirs) = std::env::var("XDG_DATA_DIRS") {
        for d in data_dirs.split(':') {
            if !d.is_empty() {
                dirs.push(PathBuf::from(d).join("applications"));
            }
        }
    } else {
        dirs.push(PathBuf::from("/usr/local/share/applications"));
        dirs.push(PathBuf::from("/usr/share/applications"));
        dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    }

    dirs
}

/// Query and parse all installed desktop applications.
pub fn list_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();
    let mut seen_ids = HashSet::new();

    for dir in application_dirs() {
        if !dir.is_dir() {
            continue;
        }

        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }

            let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };

            if seen_ids.contains(file_name) {
                continue;
            }

            if let Some(app) = parse_desktop_file(&path, file_name) {
                seen_ids.insert(file_name.to_string());
                apps.push(app);
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

/// Parse a single `.desktop` file into an [`AppInfo`].
fn parse_desktop_file(path: &Path, file_name: &str) -> Option<AppInfo> {
    let content = fs::read_to_string(path).ok()?;

    let mut in_desktop_entry = false;
    let mut name = String::new();
    let mut comment = String::new();
    let mut icon = String::new();
    let mut exec = String::new();
    let mut terminal = false;
    let mut no_display = false;
    let mut is_application = true;
    let mut keywords = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if line.starts_with('[') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim();

            match key {
                "Type" => {
                    if val != "Application" {
                        is_application = false;
                    }
                }
                "Name" if name.is_empty() => {
                    name = val.to_string();
                }
                "Comment" if comment.is_empty() => {
                    comment = val.to_string();
                }
                "Icon" if icon.is_empty() => {
                    icon = val.to_string();
                }
                "Exec" if exec.is_empty() => {
                    exec = clean_exec(val);
                }
                "Terminal" => {
                    terminal = val.eq_ignore_ascii_case("true");
                }
                "NoDisplay" | "Hidden" => {
                    if val.eq_ignore_ascii_case("true") {
                        no_display = true;
                    }
                }
                "Keywords" | "Categories" => {
                    for kw in val.split(';') {
                        let kw = kw.trim();
                        if !kw.is_empty() {
                            keywords.push(kw.to_lowercase());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if !is_application || no_display || name.is_empty() || exec.is_empty() {
        return None;
    }

    Some(AppInfo {
        id: file_name.to_string(),
        name,
        comment,
        icon,
        exec,
        terminal,
        keywords,
    })
}

/// Strip desktop action format flags (%f, %F, %u, %U, etc.) from Exec command.
fn clean_exec(raw: &str) -> String {
    let mut out = Vec::new();
    for part in raw.split_whitespace() {
        if part.starts_with('%') {
            continue;
        }
        out.push(part);
    }
    out.join(" ")
}

/// Launch an application process detached from the shell.
pub fn launch(app: &AppInfo) -> Result<(), String> {
    let mut parts: Vec<&str> = app.exec.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty exec command".to_string());
    }

    let program;
    let args;

    if app.terminal {
        let term = std::env::var("TERMINAL")
            .unwrap_or_else(|_| detect_terminal());
        program = term;
        let mut t_args = vec!["-e"];
        t_args.extend(parts);
        args = t_args;
    } else {
        program = parts.remove(0).to_string();
        args = parts;
    }

    Command::new(&program)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", app.name, e))?;

    Ok(())
}

/// Detect an available terminal emulator on the system.
fn detect_terminal() -> String {
    for term in &["ghostty", "kitty", "alacritty", "foot", "wezterm", "gnome-terminal", "konsole", "xterm"] {
        if let Ok(path) = Command::new("which").arg(term).output() {
            if path.status.success() {
                return term.to_string();
            }
        }
    }
    "xterm".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_apps_does_not_panic() {
        let apps = list_apps();
        assert!(!apps.is_empty(), "Expected at least one installed application");
    }

    #[test]
    fn test_clean_exec() {
        assert_eq!(clean_exec("firefox %u"), "firefox");
        assert_eq!(clean_exec("gimp-2.10 %U %F"), "gimp-2.10");
        assert_eq!(clean_exec("nvim"), "nvim");
    }
}
