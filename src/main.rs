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
            // explorer.exe supports UNC paths
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
    file: PathBuf,
}

fn generate_html_template(markdown_content: &str) -> String {
    // Only escape </script> to prevent breaking the script tag
    // No need for full HTML escaping since content is in type="text/plain"
    let safe_content = markdown_content.replace("</script>", "<\\/script>");

    format!(
        r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mdview - Markdown Viewer</title>

    <!-- Prism.js Theme: Tomorrow Night -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css">

    <!-- KaTeX CSS -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">

    <!-- Google Fonts: Noto Sans SC + Noto Sans Math + JetBrains Mono -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;500;600;700&family=Noto+Sans+Math&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">

    <style>
        /* ==========================================
           Claude-Style Typography & Layout
           ========================================== */

        :root {{
            /* Gruvbox Light Theme */
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
            --font-sans: "Noto Sans SC", system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            --font-math: "Noto Sans Math", "KaTeX_Main", serif;
            --font-mono: "JetBrains Mono", "Fira Code", Consolas, "Courier New", monospace;
            --max-width: 800px;
            --line-height: 1.65;
        }}

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        html {{
            font-size: 16px;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }}

        body {{
            font-family: var(--font-sans);
            line-height: var(--line-height);
            color: var(--text-primary);
            background-color: var(--bg-primary);
            padding: 3rem 2rem;
        }}

        .container {{
            max-width: var(--max-width);
            margin: 0 auto;
        }}

        /* ==========================================
           Typography Elements
           ========================================== */

        h1, h2, h3, h4, h5, h6 {{
            font-weight: 600;
            line-height: 1.3;
            margin-top: 2em;
            margin-bottom: 0.75em;
            color: var(--text-primary);
        }}

        h1 {{
            font-size: 2rem;
            font-weight: 700;
            margin-top: 0;
            padding-bottom: 0.5em;
            border-bottom: 1px solid var(--border-color);
        }}

        h2 {{
            font-size: 1.5rem;
            padding-bottom: 0.3em;
            border-bottom: 1px solid var(--border-color);
        }}

        h3 {{
            font-size: 1.25rem;
        }}

        h4 {{
            font-size: 1.1rem;
        }}

        h5, h6 {{
            font-size: 1rem;
            color: var(--text-secondary);
        }}

        p {{
            margin-bottom: 1.5em;
        }}

        a {{
            color: var(--link-color);
            text-decoration: none;
            border-bottom: 1px solid transparent;
            transition: border-bottom-color 0.2s ease;
        }}

        a:hover {{
            color: var(--link-hover-color);
            border-bottom-color: var(--link-hover-color);
        }}

        strong {{
            font-weight: 600;
        }}

        em {{
            font-style: italic;
        }}

        /* ==========================================
           Lists
           ========================================== */

        ul, ol {{
            margin-bottom: 1.5em;
            padding-left: 1.5em;
        }}

        li {{
            margin-bottom: 0.5em;
        }}

        li > ul,
        li > ol {{
            margin-top: 0.5em;
            margin-bottom: 0;
        }}

        /* ==========================================
           Blockquotes
           ========================================== */

        blockquote {{
            margin: 1.5em 0;
            padding: 0.75em 1.25em;
            border-left: 4px solid var(--accent-color);
            background-color: var(--bg-secondary);
            border-radius: 0 8px 8px 0;
            color: var(--text-secondary);
        }}

        blockquote p:last-child {{
            margin-bottom: 0;
        }}

        blockquote code {{
            background-color: rgba(0, 0, 0, 0.1);
        }}

        /* ==========================================
           Code Blocks & Inline Code
           ========================================== */

        code {{
            font-family: var(--font-mono);
            font-size: 0.875em;
            background-color: rgba(0, 0, 0, 0.08);
            padding: 0.2em 0.4em;
            border-radius: 4px;
            color: var(--text-primary);
        }}

        pre {{
            margin: 1.5em 0;
            border-radius: 8px;
            overflow: hidden;
            background-color: var(--bg-code);
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
        }}

        pre code {{
            display: block;
            padding: 1.25em 1.5em;
            overflow-x: auto;
            font-size: 0.875rem;
            line-height: 1.6;
            background-color: transparent;
            color: #ebdbb2;
            border-radius: 0;
        }}

        /* Remove Prism's default margin */
        pre[class*="language-"] {{
            margin: 1.5em 0;
        }}

        code[class*="language-"] {{
            background-color: transparent;
            padding: 0;
        }}

        /* ==========================================
           Tables
           ========================================== */

        table {{
            width: 100%;
            margin: 1.5em 0;
            border-collapse: collapse;
            font-size: 0.95rem;
        }}

        thead {{
            background-color: var(--bg-secondary);
        }}

        th, td {{
            padding: 0.75em 1em;
            text-align: left;
            border-bottom: 1px solid var(--border-color);
        }}

        th {{
            font-weight: 600;
            color: var(--text-primary);
        }}

        tbody tr:hover {{
            background-color: rgba(0, 0, 0, 0.05);
        }}

        /* ==========================================
           Horizontal Rule
           ========================================== */

        hr {{
            margin: 2em 0;
            border: none;
            border-top: 1px solid var(--border-color);
        }}

        /* ==========================================
           Images
           ========================================== */

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 1em 0;
        }}

        /* ==========================================
           KaTeX Math Formula Styles
           ========================================== */

        .katex-display {{
            margin: 1.5em 0;
            overflow-x: auto;
            overflow-y: hidden;
            padding: 0.5em 0;
            font-family: var(--font-math);
        }}

        .katex {{
            font-size: 1.1em;
            font-family: var(--font-math);
        }}

        .katex .mathnormal,
        .katex .mathit {{
            font-family: var(--font-math);
            font-style: normal;
        }}

        .katex .mathnormal {{
            font-style: normal;
        }}

        .katex {{
            font-style: normal;
        }}

        /* Inline math alignment */
        .katex-inline {{
            vertical-align: baseline;
        }}

        /* Block math */
        .katex-block {{
            display: flex;
            justify-content: center;
            margin: 1.5em 0;
            padding: 1em;
            background-color: var(--bg-secondary);
            border-radius: 8px;
        }}

        /* ==========================================
           Task Lists (GitHub-style)
           ========================================== */

        .task-list-item {{
            list-style: none;
            margin-left: -1.5em;
        }}

        .task-list-item input {{
            margin-right: 0.5em;
        }}

        /* ==========================================
           Print Styles
           ========================================== */

        @media print {{
            body {{
                background-color: white;
                padding: 0;
            }}

            pre {{
                box-shadow: none;
                border: 1px solid var(--border-color);
            }}
        }}

        /* ==========================================
           Responsive Adjustments
           ========================================== */

        @media (max-width: 640px) {{
            body {{
                padding: 1.5rem 1rem;
            }}

            h1 {{
                font-size: 1.75rem;
            }}

            h2 {{
                font-size: 1.35rem;
            }}

            pre code {{
                font-size: 0.8rem;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div id="content"></div>
    </div>

    <!-- Markdown Content (safely stored in script tag) -->
    <script id="markdown-source" type="text/plain">{safe_content}</script>

    <!-- Marked.js -->
    <script src="https://cdn.jsdelivr.net/npm/marked@16.3.0/lib/marked.umd.min.js"></script>

    <!-- KaTeX -->
    <script src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>

    <!-- Marked KaTeX Extension -->
    <script src="https://cdn.jsdelivr.net/npm/marked-katex-extension@5.1.10/lib/index.umd.min.js"></script>

    <!-- Prism.js Core -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>

    <!-- Prism.js Language Components -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-c.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-cpp.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-python.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-javascript.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-typescript.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-bash.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-json.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-markdown.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-yaml.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-sql.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-java.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-go.min.js"></script>

    <script>
        // Get markdown content from script tag
        const markdownSource = document.getElementById('markdown-source');
        const markdownText = markdownSource.textContent || markdownSource.innerText;

        // Configure marked with KaTeX extension
        marked.use(markedKatex({{
            throwOnError: false,
            output: 'html',
            nonStandard: true
        }}));

        // Parse and render markdown
        const contentElement = document.getElementById('content');
        contentElement.innerHTML = marked.parse(markdownText);

        // Apply Prism.js syntax highlighting to code blocks
        Prism.highlightAllUnder(contentElement);
    </script>
</body>
</html>"##
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Validate file exists
    if !cli.file.exists() {
        eprintln!("Error: File '{}' not found", cli.file.display());
        std::process::exit(1);
    }

    // Validate file extension
    if let Some(ext) = cli.file.extension() {
        if ext != "md" && ext != "markdown" {
            eprintln!("Warning: File does not have a .md or .markdown extension");
        }
    }

    // Read markdown file
    let markdown_content = fs::read_to_string(&cli.file)?;

    // Generate HTML
    let html_content = generate_html_template(&markdown_content);

    // Generate random filename for temp file
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen();
    let temp_filename = format!("mdview_{:08x}.html", random_id);
    let temp_path = std::env::temp_dir().join(temp_filename);

    // Write HTML to temp file
    fs::write(&temp_path, &html_content)?;

    // Open in default browser
    let path_str = temp_path.to_str().ok_or("Invalid temp path")?;
    let url = format!("file://{}", path_str);

    // Try multiple methods to open the browser
    let opened = if is_wsl() {
        open_in_wsl(&temp_path)
    } else {
        open::that(&url).is_ok()
    };

    if !opened {
        // Fallback: try common browser commands
        let browsers = ["xdg-open", "firefox", "google-chrome", "chromium"];
        for browser in &browsers {
            if Command::new(browser).arg(&url).spawn().is_ok() {
                break;
            }
        }
    }

    // Print success message
    println!("✓ Markdown rendered successfully!");
    println!("  File: {}", temp_path.display());
    println!("  URL:  {}", url);

    // Small delay to ensure browser has time to open the file
    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}
