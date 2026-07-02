use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FontValue {
    Single(String),
    Multiple(Vec<String>),
}

impl FontValue {
    pub fn into_vec(self) -> Vec<String> {
        match self {
            FontValue::Single(s) => vec![s],
            FontValue::Multiple(v) => v,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FontConfig {
    pub sans: Option<FontValue>,
    pub mono: Option<FontValue>,
    pub math: Option<FontValue>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub fonts: Option<FontConfig>,
}

pub static DEFAULT_FONT_SANS: &[&str] = &[
    "Noto Sans SC",
    "system-ui",
    "-apple-system",
    "Segoe UI",
    "Roboto",
    "Helvetica Neue",
    "Arial",
    "sans-serif",
];

pub static DEFAULT_FONT_MONO: &[&str] = &[
    "JetBrains Mono",
    "Fira Code",
    "ui-monospace",
    "SF Mono",
    "Cascadia Code",
    "Consolas",
    "Courier New",
    "monospace",
];

pub static DEFAULT_FONT_MATH: &[&str] = &["KaTeX_Main", "serif"];

impl Config {
    pub fn load() -> Self {
        let path = get_config_path();
        if let Some(path) = path {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Config { fonts: None }
    }

    pub fn font_sans(&self) -> Vec<String> {
        build_stack(
            self.fonts.as_ref().and_then(|f| f.sans.clone()),
            DEFAULT_FONT_SANS,
        )
    }

    pub fn font_mono(&self) -> Vec<String> {
        build_stack(
            self.fonts.as_ref().and_then(|f| f.mono.clone()),
            DEFAULT_FONT_MONO,
        )
    }

    pub fn font_math(&self) -> Vec<String> {
        build_stack(
            self.fonts.as_ref().and_then(|f| f.math.clone()),
            DEFAULT_FONT_MATH,
        )
    }

    pub fn preferred_fonts(&self) -> Vec<String> {
        let mut fonts = Vec::new();
        if let Some(ref fc) = self.fonts {
            if let Some(ref sans) = fc.sans {
                fonts.extend(sans.clone().into_vec());
            }
            if let Some(ref mono) = fc.mono {
                fonts.extend(mono.clone().into_vec());
            }
            if let Some(ref math) = fc.math {
                fonts.extend(math.clone().into_vec());
            }
        }
        if fonts.is_empty() {
            fonts.push(DEFAULT_FONT_SANS[0].to_string());
            fonts.push(DEFAULT_FONT_MONO[0].to_string());
            fonts.push(DEFAULT_FONT_MATH[0].to_string());
        }
        fonts
    }
}

fn get_config_path() -> Option<std::path::PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(".config").join("mdview.toml"))
}

fn build_stack(preferred: Option<FontValue>, defaults: &[&str]) -> Vec<String> {
    let mut stack: Vec<String> = match preferred {
        Some(v) => v.into_vec(),
        None => Vec::new(),
    };
    for font in defaults {
        let s = font.to_string();
        if !stack.contains(&s) {
            stack.push(s);
        }
    }
    stack
}
