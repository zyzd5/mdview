# mdview

A minimal, high-performance CLI tool that renders Markdown files in your browser with browser.

## Features

- Multiple color schemes: gruvbox-light, gruvbox-dark, nord, catppuccin-latte, catppuccin-mocha
- Smart font fallback:
  - **Noto Sans SC** for Chinese text (falls back to system UI fonts)
  - **JetBrains Mono / Fira Code** for code (falls back to system monospace fonts)
  - Configurable via `~/.config/mdview.toml`
- KaTeX for LaTeX math formulas (fonts embedded as data URIs, fully offline)
- Prism.js syntax highlighting (Python, C++, Rust, etc.)
- Single-file HTML output, no server needed
- `--checkhealth` to diagnose font availability
- `--debug` to show font detection info in rendered output

## Install

```bash
cargo install --path .
```

## Usage

```bash
mdview <file.md>                            # render and open in browser
mdview --checkhealth                        # check if preferred fonts are installed
mdview --debug <file.md>                    # render with font debug info at page bottom
mdview --colorscheme nord <file.md>         # use a specific color scheme
mdview --colorscheme bogus <file.md>        # error: lists available schemes
mdview completion <SHELL>                   # generate shell completion script
```

## Shell Completion

Generate and install completion scripts for your shell:

```bash
# Bash
mdview completion bash > /usr/local/share/bash-completion/completions/mdview
# Zsh
mdview completion zsh > /usr/local/share/zsh/site-functions/_mdview
# Fish
mdview completion fish > ~/.config/fish/completions/mdview.fish
```

After reloading your shell (or running `source`), you can use:
- `mdview --<Tab>` to cycle through `--checkhealth`, `--debug`, `--colorscheme`
- `mdview completion <Tab>` to pick between bash, zsh, fish, etc.
- Run `mdview --checkhealth` to see whether completion scripts are installed.

## Configuration

Font preferences and color scheme can be set in `~/.config/mdview.toml`:

```toml
colorscheme = "catppuccin-mocha"       # default: gruvbox-light

[fonts]
sans = "Source Han Sans SC"            # prefer a different Chinese font
mono = ["Iosevka", "JetBrains Mono"]   # list multiple preferences
math = "Noto Sans Math"
```

If a font is not installed, the browser automatically falls back through the stack (system-ui → platform defaults → generic families).

## Dependencies

- [marked.js](https://github.com/markedjs/marked) - Markdown parser
- [KaTeX](https://github.com/KaTeX/KaTeX) - Math rendering
- [Prism.js](https://github.com/PrismJS/prism) - Syntax highlighting

## License

MIT
