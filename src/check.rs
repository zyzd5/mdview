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

fn check_completion() -> Vec<(&'static str, bool)> {
    let locations: &[(&str, &str)] = &[
        ("bash", "/usr/local/share/bash-completion/completions/mdview"),
        ("bash", "/etc/bash_completion.d/mdview"),
        ("zsh", "/usr/local/share/zsh/site-functions/_mdview"),
        ("fish", "/usr/local/share/fish/vendor_completions.d/mdview.fish"),
        ("fish", "~/.config/fish/completions/mdview.fish"),
    ];

    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for (shell, path) in locations {
        let expanded = if path.starts_with('~') {
            let home = dirs::home_dir().map(|p| p.to_string_lossy().to_string());
            match home {
                Some(h) => path.replacen('~', &h, 1),
                None => continue,
            }
        } else {
            (*path).to_string()
        };

        let exists = std::path::Path::new(&expanded).exists();
        if exists && !seen.contains(shell) {
            seen.insert(shell);
            results.push((*shell, true));
        }
    }

    for shell in &["bash", "zsh", "fish"] {
        if !seen.contains(shell) {
            results.push((*shell, false));
        }
    }

    results.sort_by_key(|(s, _)| *s);
    results
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

    println!("\nShell Completion Check");
    println!("{}", "─".repeat(50));
    let completion_all = check_completion();
    for (shell, installed) in &completion_all {
        let icon = if *installed { "✓" } else { "✗" };
        let hint = if *installed { "" } else { "  (run: mdview completion <shell> > ...)" };
        println!(" {}  {}{}", icon, shell, hint);
    }
    if completion_all.iter().any(|(_, ok)| *ok) {
        println!("\nCompletion scripts are installed for some shells.");
    } else {
        println!("\nNo completion scripts found. Run 'mdview completion --help' to get started.");
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
