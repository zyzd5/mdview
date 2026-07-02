mod check;
mod config;

use clap::Parser;
use rand::Rng;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn is_wsl() -> bool {
    if let Ok(version) = fs::read_to_string("/proc/version") {
        let v = version.to_lowercase();
        return v.contains("microsoft") || v.contains("wsl");
    }
    false
}

fn open_in_wsl(path: &std::path::Path) -> bool {
    let output = Command::new("wslpath").arg("-w").arg(path).output();
    if let Ok(output) = output {
        if output.status.success() {
            let win_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Command::new("explorer.exe")
                .arg(&win_path)
                .spawn()
                .is_ok();
        }
    }
    false
}

#[derive(Parser)]
#[command(name = "mdview", about = "Render Markdown files with Claude-style typography")]
struct Cli {
    /// Path to the Markdown file
    #[arg(required_unless_present = "checkhealth")]
    file: Option<PathBuf>,

    /// Check system fonts availability without rendering
    #[arg(long)]
    checkhealth: bool,

    /// Append font debug info to the rendered output
    #[arg(long)]
    debug: bool,
}

fn generate_katex_font_faces() -> String {
    use base64::Engine;

    let fonts: &[(&str, u16, &str, &[u8])] = &[
        (
            "KaTeX_AMS",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_AMS-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Caligraphic",
            700,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Caligraphic-Bold.woff2") as &[u8],
        ),
        (
            "KaTeX_Caligraphic",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Caligraphic-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Fraktur",
            700,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Fraktur-Bold.woff2") as &[u8],
        ),
        (
            "KaTeX_Fraktur",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Fraktur-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Main",
            700,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Main-Bold.woff2") as &[u8],
        ),
        (
            "KaTeX_Main",
            700,
            "italic",
            include_bytes!("../vendor/fonts/KaTeX_Main-BoldItalic.woff2") as &[u8],
        ),
        (
            "KaTeX_Main",
            400,
            "italic",
            include_bytes!("../vendor/fonts/KaTeX_Main-Italic.woff2") as &[u8],
        ),
        (
            "KaTeX_Main",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Main-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Math",
            700,
            "italic",
            include_bytes!("../vendor/fonts/KaTeX_Math-BoldItalic.woff2") as &[u8],
        ),
        (
            "KaTeX_Math",
            400,
            "italic",
            include_bytes!("../vendor/fonts/KaTeX_Math-Italic.woff2") as &[u8],
        ),
        (
            "KaTeX_SansSerif",
            700,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_SansSerif-Bold.woff2") as &[u8],
        ),
        (
            "KaTeX_SansSerif",
            400,
            "italic",
            include_bytes!("../vendor/fonts/KaTeX_SansSerif-Italic.woff2") as &[u8],
        ),
        (
            "KaTeX_SansSerif",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_SansSerif-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Script",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Script-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Size1",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Size1-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Size2",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Size2-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Size3",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Size3-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Size4",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Size4-Regular.woff2") as &[u8],
        ),
        (
            "KaTeX_Typewriter",
            400,
            "normal",
            include_bytes!("../vendor/fonts/KaTeX_Typewriter-Regular.woff2") as &[u8],
        ),
    ];

    let mut css = String::new();
    for (family, weight, style, data) in fonts {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        css.push_str(&format!(
            "@font-face{{font-family:{};font-style:{};font-weight:{};src:url(data:font/woff2;base64,{})format(\"woff2\")}}",
            family, style, weight, encoded
        ));
    }
    css
}

fn css_font_family(fonts: &[String]) -> String {
    let generic: &[&str] = &[
        "serif",
        "sans-serif",
        "monospace",
        "cursive",
        "fantasy",
        "system-ui",
        "ui-serif",
        "ui-sans-serif",
        "ui-monospace",
        "ui-rounded",
        "math",
        "emoji",
        "fangsong",
    ];

    fonts
        .iter()
        .map(|f| {
            if generic.contains(&f.as_str()) || !f.contains(' ') {
                f.clone()
            } else {
                format!("\"{}\"", f)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_html_template(
    markdown_content: &str,
    font_sans: &[String],
    font_mono: &[String],
    font_math: &[String],
    katex_font_css: &str,
    debug: bool,
) -> String {
    let debug_content = if debug {
        let mut extra = String::new();
        extra.push_str(
            "\n\n---\n\n## Debug: Font Information\n\n<div id=\"font-debug\"><noscript>JavaScript required.</noscript></div>\n",
        );
        extra
    } else {
        String::new()
    };
    let safe_content = markdown_content.replace("</script>", "<\\/script>") + &debug_content;

    let prism_css = include_str!("../vendor/prism-tomorrow.min.css");
    let katex_css = include_str!("../vendor/katex.min.css");
    let marked_js = include_str!("../vendor/marked.umd.min.js");
    let katex_js = include_str!("../vendor/katex.min.js");
    let marked_katex_js = include_str!("../vendor/marked-katex-extension.min.js");
    let prism_js = include_str!("../vendor/prism.min.js");
    let prism_c = include_str!("../vendor/prism-c.min.js");
    let prism_cpp = include_str!("../vendor/prism-cpp.min.js");
    let prism_python = include_str!("../vendor/prism-python.min.js");
    let prism_rust = include_str!("../vendor/prism-rust.min.js");
    let prism_javascript = include_str!("../vendor/prism-javascript.min.js");
    let prism_typescript = include_str!("../vendor/prism-typescript.min.js");
    let prism_bash = include_str!("../vendor/prism-bash.min.js");
    let prism_json = include_str!("../vendor/prism-json.min.js");
    let prism_markdown = include_str!("../vendor/prism-markdown.min.js");
    let prism_yaml = include_str!("../vendor/prism-yaml.min.js");
    let prism_sql = include_str!("../vendor/prism-sql.min.js");
    let prism_java = include_str!("../vendor/prism-java.min.js");
    let prism_go = include_str!("../vendor/prism-go.min.js");

    let sans_family = css_font_family(font_sans);
    let mono_family = css_font_family(font_mono);
    let math_family = css_font_family(font_math);

    let mut html = String::with_capacity(
        safe_content.len()
            + prism_css.len()
            + katex_css.len()
            + katex_font_css.len()
            + marked_js.len()
            + katex_js.len()
            + marked_katex_js.len()
            + prism_js.len()
            + 21000,
    );

    html.push_str(r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mdview - Markdown Viewer</title>

    <!-- KaTeX Fonts (embedded as data URIs) -->
    <style>"##);
    html.push_str(katex_font_css);
    html.push_str("</style>\n\n    <!-- Prism.js Theme: Tomorrow Night -->\n    <style>");
    html.push_str(prism_css);
    html.push_str("</style>\n\n    <!-- KaTeX CSS -->\n    <style>");
    html.push_str(katex_css);
    html.push_str("</style>\n\n    <style>");
    html.push_str(r##"
        :root {
            --bg-primary: #fbf1c7;
            --bg-secondary: #ebdbb2;
            --bg-code: #282828;
            --text-primary: #3c3836;
            --text-secondary: #504945;
            --text-muted: #7c6f64;
            --border-color: #d5c4a1;
            --accent-color: #d65d0e;
            --link-color: #458588;
            --link-hover-color: #076678;
            --font-sans: "##);
    html.push_str(&sans_family);
    html.push_str(r##";
            --font-math: "##);
    html.push_str(&math_family);
    html.push_str(r##";
            --font-mono: "##);
    html.push_str(&mono_family);
    html.push_str(r##";
            --max-width: 800px;
            --line-height: 1.65;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        html {
            font-size: 16px;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }

        body {
            font-family: var(--font-sans);
            line-height: var(--line-height);
            color: var(--text-primary);
            background-color: var(--bg-primary);
            padding: 3rem 2rem;
        }

        .container {
            max-width: var(--max-width);
            margin: 0 auto;
        }

        h1, h2, h3, h4, h5, h6 {
            font-weight: 600;
            line-height: 1.3;
            margin-top: 2em;
            margin-bottom: 0.75em;
            color: var(--text-primary);
        }

        h1 {
            font-size: 2rem;
            font-weight: 700;
            margin-top: 0;
            padding-bottom: 0.5em;
            border-bottom: 1px solid var(--border-color);
        }

        h2 {
            font-size: 1.5rem;
            padding-bottom: 0.3em;
            border-bottom: 1px solid var(--border-color);
        }

        h3 {
            font-size: 1.25rem;
        }

        h4 {
            font-size: 1.1rem;
        }

        h5, h6 {
            font-size: 1rem;
            color: var(--text-secondary);
        }

        p {
            margin-bottom: 1.5em;
        }

        a {
            color: var(--link-color);
            text-decoration: none;
            border-bottom: 1px solid transparent;
            transition: border-bottom-color 0.2s ease;
        }

        a:hover {
            color: var(--link-hover-color);
            border-bottom-color: var(--link-hover-color);
        }

        strong {
            font-weight: 600;
        }

        em {
            font-style: italic;
        }

        ul, ol {
            margin-bottom: 1.5em;
            padding-left: 1.5em;
        }

        li {
            margin-bottom: 0.5em;
        }

        li > ul,
        li > ol {
            margin-top: 0.5em;
            margin-bottom: 0;
        }

        blockquote {
            margin: 1.5em 0;
            padding: 0.75em 1.25em;
            border-left: 4px solid var(--accent-color);
            background-color: var(--bg-secondary);
            border-radius: 0 8px 8px 0;
            color: var(--text-secondary);
        }

        blockquote p:last-child {
            margin-bottom: 0;
        }

        blockquote code {
            background-color: rgba(0, 0, 0, 0.1);
        }

        code {
            font-family: var(--font-mono);
            font-size: 0.875em;
            background-color: rgba(0, 0, 0, 0.08);
            padding: 0.2em 0.4em;
            border-radius: 4px;
            color: var(--text-primary);
        }

        pre {
            margin: 1.5em 0;
            border-radius: 8px;
            overflow: hidden;
            background-color: var(--bg-code);
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
        }

        pre code {
            display: block;
            padding: 1.25em 1.5em;
            overflow-x: auto;
            font-size: 0.875rem;
            line-height: 1.6;
            background-color: transparent;
            color: #ebdbb2;
            border-radius: 0;
        }

        pre[class*="language-"] {
            margin: 1.5em 0;
        }

        code[class*="language-"] {
            background-color: transparent;
            padding: 0;
        }

        table {
            width: 100%;
            margin: 1.5em 0;
            border-collapse: collapse;
            font-size: 0.95rem;
        }

        thead {
            background-color: var(--bg-secondary);
        }

        th, td {
            padding: 0.75em 1em;
            text-align: left;
            border-bottom: 1px solid var(--border-color);
        }

        th {
            font-weight: 600;
            color: var(--text-primary);
        }

        tbody tr:hover {
            background-color: rgba(0, 0, 0, 0.05);
        }

        hr {
            margin: 2em 0;
            border: none;
            border-top: 1px solid var(--border-color);
        }

        img {
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 1em 0;
        }

        .katex-display {
            margin: 1.5em 0;
            overflow-x: auto;
            overflow-y: hidden;
            padding: 0.5em 0;
            font-family: var(--font-math);
        }

        .katex {
            font-size: 1.1em;
            font-family: var(--font-math);
        }

        .katex .mathnormal,
        .katex .mathit {
            font-family: var(--font-math);
            font-style: normal;
        }

        .katex .mathnormal {
            font-style: normal;
        }

        .katex {
            font-style: normal;
        }

        .katex-inline {
            vertical-align: baseline;
        }

        .katex-block {
            display: flex;
            justify-content: center;
            margin: 1.5em 0;
            padding: 1em;
            background-color: var(--bg-secondary);
            border-radius: 8px;
        }

        .task-list-item {
            list-style: none;
            margin-left: -1.5em;
        }

        .task-list-item input {
            margin-right: 0.5em;
        }

        @media print {
            body {
                background-color: white;
                padding: 0;
            }

            pre {
                box-shadow: none;
                border: 1px solid var(--border-color);
            }
        }

        @media (max-width: 640px) {
            body {
                padding: 1.5rem 1rem;
            }

            h1 {
                font-size: 1.75rem;
            }

            h2 {
                font-size: 1.35rem;
            }

            pre code {
                font-size: 0.8rem;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div id="content"></div>
    </div>

    <script id="markdown-source" type="text/plain">"##);
    html.push_str(&safe_content);
    html.push_str(r##"</script>

    <script>"##);
    html.push_str(marked_js);
    html.push_str("</script>\n\n    <script>");
    html.push_str(katex_js);
    html.push_str("</script>\n\n    <script>");
    html.push_str(marked_katex_js);
    html.push_str("</script>\n\n    <script>");
    html.push_str(prism_js);
    html.push_str("</script>\n\n    <script>");
    html.push_str(prism_c);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_cpp);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_python);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_rust);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_javascript);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_typescript);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_bash);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_json);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_markdown);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_yaml);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_sql);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_java);
    html.push_str("</script>\n    <script>");
    html.push_str(prism_go);
    html.push_str(r##"</script>

    <script>
        const markdownSource = document.getElementById('markdown-source');
        const markdownText = markdownSource.textContent || markdownSource.innerText;

        marked.use(markedKatex({
            throwOnError: false,
            output: 'html',
            nonStandard: true
        }));

        const contentElement = document.getElementById('content');
        contentElement.innerHTML = marked.parse(markdownText);

        Prism.highlightAllUnder(contentElement);

        (function() {
            var el = document.getElementById('font-debug');
            if (!el) return;
            document.fonts.ready.then(function() {
                function check(family) {
                    try {
                        var q = family.indexOf(' ') >= 0 ? '"' + family + '"' : family;
                        return document.fonts.check('12px ' + q) ? '\u2713' : '\u2717';
                    } catch(e) { return '?'; }
                }
                var roles = [
                    {label: 'Body (Sans)', sel: 'body'},
                    {label: 'Code (Mono)', sel: 'code'},
                    {label: 'Math', sel: '.katex'}
                ];
                var html = '<table><tr><th>Role</th><th>Font Checks</th></tr>';
                roles.forEach(function(r) {
                    var target = document.querySelector(r.sel);
                    if (!target) return;
                    var stack = getComputedStyle(target).fontFamily;
                    var fonts = stack.split(',').map(function(f) {
                        return f.trim().replace(/["']/g, '');
                    });
                    var seen = {};
                    var unique = fonts.filter(function(f) { return seen[f] ? false : (seen[f] = true); });
                    var checks = unique.map(function(f) {
                        return f + ' ' + check(f);
                    }).join(' \u2192 ');
                    html += '<tr><td>' + r.label + '</td><td style="font-size:0.85em">' + checks + '</td></tr>';
                });
                html += '</table>';
                el.innerHTML = html;
            });
        })();
    </script>
</body>
</html>"##);

    html
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = config::Config::load();

    if cli.checkhealth {
        let preferred = config.preferred_fonts();
        check::print_health_report(&preferred);
        return Ok(());
    }

    let file = cli.file.unwrap();

    if !file.exists() {
        eprintln!("Error: File '{}' not found", file.display());
        std::process::exit(1);
    }

    if let Some(ext) = file.extension() {
        if ext != "md" && ext != "markdown" {
            eprintln!("Warning: File does not have a .md or .markdown extension");
        }
    }

    let markdown_content = fs::read_to_string(&file)?;

    let katex_font_css = generate_katex_font_faces();
    let font_sans = config.font_sans();
    let font_mono = config.font_mono();
    let font_math = config.font_math();

    let html_content = generate_html_template(
        &markdown_content,
        &font_sans,
        &font_mono,
        &font_math,
        &katex_font_css,
        cli.debug,
    );

    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen();
    let temp_filename = format!("mdview_{:08x}.html", random_id);
    let temp_path = std::env::temp_dir().join(temp_filename);

    fs::write(&temp_path, &html_content)?;

    let path_str = temp_path.to_str().ok_or("Invalid temp path")?;
    let url = format!("file://{}", path_str);

    let opened = if is_wsl() {
        open_in_wsl(&temp_path)
    } else {
        open::that(&url).is_ok()
    };

    if !opened {
        let browsers = ["xdg-open", "firefox", "google-chrome", "chromium"];
        for browser in &browsers {
            if Command::new(browser).arg(&url).spawn().is_ok() {
                break;
            }
        }
    }

    println!("✓ Markdown rendered successfully!");
    println!("  File: {}", temp_path.display());
    println!("  URL:  {}", url);

    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}
