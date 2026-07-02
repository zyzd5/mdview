use std::path::PathBuf;

pub struct FontStatus {
    pub family: String,
    pub installed: bool,
}

pub fn check_font(family: &str) -> bool {
    let search_name = family.replace(" ", "").to_lowercase();
    for dir in font_directories() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        let is_font = matches!(
                            ext_str.to_lowercase().as_str(),
                            "ttf" | "otf" | "ttc" | "woff" | "woff2"
                        );
                        if is_font {
                            let name = path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_lowercase()
                                .replace(" ", "");
                            if name.contains(&search_name) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn check_fonts(families: &[String]) -> Vec<FontStatus> {
    families
        .iter()
        .map(|family| FontStatus {
            family: family.clone(),
            installed: check_font(family),
        })
        .collect()
}

pub fn print_health_report(families: &[String]) {
    let statuses = check_fonts(families);
    println!("\nFont Check Report");
    println!("{}", "─".repeat(50));

    let all_ok = statuses.iter().all(|s| s.installed);

    for status in &statuses {
        let icon = if status.installed { "✓" } else { "✗" };
        println!(
            " {}  {}  {}",
            icon,
            status.family,
            if status.installed {
                "Installed"
            } else {
                "Not installed"
            }
        );
    }

    println!();
    if all_ok {
        println!("All configured fonts are available.");
    } else {
        println!("Some fonts are missing. The browser will fall back to system fonts.");
    }
}

fn font_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(home) = dirs::home_dir() {
        // Linux / general
        dirs.push(home.join(".fonts"));
        dirs.push(home.join(".local").join("share").join("fonts"));
        // macOS user fonts
        dirs.push(home.join("Library").join("Fonts"));
    }

    // macOS system fonts
    dirs.push(PathBuf::from("/Library/Fonts"));
    dirs.push(PathBuf::from("/System/Library/Fonts"));
    // Linux system fonts
    dirs.push(PathBuf::from("/usr/share/fonts"));
    dirs.push(PathBuf::from("/usr/local/share/fonts"));

    dirs
}
