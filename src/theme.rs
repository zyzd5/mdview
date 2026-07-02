pub struct Theme {
    pub name: &'static str,
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_code: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub border_color: &'static str,
    pub accent_color: &'static str,
    pub link_color: &'static str,
    pub link_hover_color: &'static str,
}

impl Theme {
    pub fn by_name(name: &str) -> Option<&'static Theme> {
        THEMES.iter().find(|t| t.name == name)
    }

    pub fn css_vars(&self) -> String {
        format!(
            "--bg-primary: {};\n            --bg-secondary: {};\n            --bg-code: {};\n            --text-primary: {};\n            --text-secondary: {};\n            --text-muted: {};\n            --border-color: {};\n            --accent-color: {};\n            --link-color: {};\n            --link-hover-color: {}",
            self.bg_primary,
            self.bg_secondary,
            self.bg_code,
            self.text_primary,
            self.text_secondary,
            self.text_muted,
            self.border_color,
            self.accent_color,
            self.link_color,
            self.link_hover_color,
        )
    }

    pub fn available_names() -> Vec<&'static str> {
        THEMES.iter().map(|t| t.name).collect()
    }
}

pub const THEMES: [Theme; 5] = [
    Theme {
        name: "gruvbox-light",
        bg_primary: "#fbf1c7",
        bg_secondary: "#ebdbb2",
        bg_code: "#282828",
        text_primary: "#3c3836",
        text_secondary: "#504945",
        text_muted: "#7c6f64",
        border_color: "#d5c4a1",
        accent_color: "#d65d0e",
        link_color: "#458588",
        link_hover_color: "#076678",
    },
    Theme {
        name: "gruvbox-dark",
        bg_primary: "#282828",
        bg_secondary: "#3c3836",
        bg_code: "#1d2021",
        text_primary: "#ebdbb2",
        text_secondary: "#d5c4a1",
        text_muted: "#928374",
        border_color: "#504945",
        accent_color: "#d65d0e",
        link_color: "#83a598",
        link_hover_color: "#458588",
    },
    Theme {
        name: "nord",
        bg_primary: "#2e3440",
        bg_secondary: "#3b4252",
        bg_code: "#1e222a",
        text_primary: "#eceff4",
        text_secondary: "#d8dee9",
        text_muted: "#7b88a1",
        border_color: "#4c566a",
        accent_color: "#bf616a",
        link_color: "#88c0d0",
        link_hover_color: "#81a1c1",
    },
    Theme {
        name: "catppuccin-latte",
        bg_primary: "#eff1f5",
        bg_secondary: "#e6e9ef",
        bg_code: "#dce0e8",
        text_primary: "#4c4f69",
        text_secondary: "#5c5f77",
        text_muted: "#9ca0b0",
        border_color: "#ccd0da",
        accent_color: "#fe640b",
        link_color: "#1e66f5",
        link_hover_color: "#04a5e5",
    },
    Theme {
        name: "catppuccin-mocha",
        bg_primary: "#1e1e2e",
        bg_secondary: "#313244",
        bg_code: "#11111b",
        text_primary: "#cdd6f4",
        text_secondary: "#bac2de",
        text_muted: "#7f849c",
        border_color: "#45475a",
        accent_color: "#fab387",
        link_color: "#89b4fa",
        link_hover_color: "#89dceb",
    },
];
