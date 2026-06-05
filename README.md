* PURE VIBE CODING BY MIMO V2.5 PRO
# mdview

A minimal, high-performance CLI tool that renders Markdown files with in your browser.

## Features

- Gruvbox color scheme
- Noto Sans SC for Chinese text
- KaTeX for LaTeX math formulas
- Prism.js syntax highlighting (Python, C++, Rust, etc.)
- Single-file output, no server needed

## Install

```bash
cargo install --path .
```

## Usage

```bash
mdview <file.md>
```

## Dependencies

- [marked.js](https://github.com/markedjs/marked) - Markdown parser
- [KaTeX](https://github.com/KaTeX/KaTeX) - Math rendering
- [Prism.js](https://github.com/PrismJS/prism) - Syntax highlighting
- [Noto Sans SC](https://fonts.google.com/noto/specimen/Noto+Sans+SC) - Chinese font
- [Noto Sans Math](https://fonts.google.com/noto/specimen/Noto+Sans+Math) - Math font

## License

MIT
