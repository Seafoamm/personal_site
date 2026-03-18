use axum::http::Uri;
use maud::{DOCTYPE, Markup, Render, html};
use std::fs::File;
use std::io::{self, BufRead};
use typed_builder::TypedBuilder;

const RESUME_FILEPATH: &str = "./assets/resume.md";
const TERMINAL_PROMPT: &str = "$ ";
const TERMINAL_DEFAULT_FONT_WEIGHT: i32 = 400;
const TERMINAL_DEFAULT_OPACITY: i32 = 1;

#[derive(TypedBuilder)]
struct TerminalOutputOptions {
    #[builder(default=TERMINAL_DEFAULT_FONT_WEIGHT)]
    weight: i32,
    #[builder(default=TERMINAL_DEFAULT_OPACITY)]
    opacity: i32,
    #[builder(default = false)]
    is_italic: bool,
    #[builder(default = false)]
    is_bolded: bool,
    #[builder(default = false)]
    is_banner: bool,
    #[builder(default = false)]
    is_command: bool,
    #[builder(default = false)]
    is_pill: bool,
    #[builder(default = false)]
    is_uri: bool,
    #[builder(default = false)]
    is_list: bool,
    #[builder(default = false)]
    is_last: bool,
}

fn tmpl_terminal_command<T: Render>(content: T, options: TerminalOutputOptions) -> Markup {
    // TODO: implement terminal options
    let cursor_class = if options.is_last { "cursor-prompt" } else { "" };
    html! {
        span.terminal_prompt.(cursor_class) { (TERMINAL_PROMPT) (content) }
    }
}

pub(crate) fn tmpl_nav_button(is_highlighted: bool, href: &str, label: &str) -> Markup {
    let active_class: &str = if is_highlighted {
        "term-window-nav-active"
    } else {
        ""
    };

    html! {
        a.(active_class) href=(href) { (label) }
    }
}

pub(crate) fn tmpl_global_chrome_wrapper(uri: Uri, body: Markup) -> Markup {
    let path = uri.path();
    let home_nav_button = tmpl_nav_button(matches!(path, "/about" | "" | "/"), "/about", "About");
    let system_design_nav_button = tmpl_nav_button(
        matches!(path, "/system_design"),
        "/system_design",
        "System Design",
    );
    let algorithms_nav_button =
        tmpl_nav_button(matches!(path, "/algorithms"), "/algorithms", "Algorithms");
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                link href="/styles.css" rel="stylesheet";
                link
                    rel="icon"
                    type="image/png"
                    href="./favicon.png"
                    script
                    src="https://unpkg.com/hyperscript.org@0.9.14" {}
                title { "Brandon Lu - Resume" }
            }
            body."clear-spacing" {
                header."term-window-title-chrome" {
                    div."term-window-title-left" {
                        ul."term-window-title-socials clear-spacing" {
                            a href="https://www.youtube.com/@DoubleColon11" {
                                img src="./yt_icon.png";
                            }
                            a href="https://linktr.ee/doublecolon11" {
                                img src="./linktree_icon.webp";
                            }
                        }
                    }
                    div."term-window-title-center" {
                        a href="." {
                            h1 { "Lu::Brandon" }
                            p { "( Brandon Lu )" }
                        }
                    }
                    p."term-window-title-right clear-spacing" { "Terminal UI inspired resume" }
                }
                nav."term-window-nav" {
                    ul."term-window-nav-tab-bar" {
                        (home_nav_button)
                        (system_design_nav_button)
                        (algorithms_nav_button)
                    }
                }
                hr;
                (body)
            }
        }
    }
}

pub(crate) fn tmpl_page_body(lines: Vec<Markup>) -> Markup {
    html! {
        ul."terminal_lines" {
            @for line in lines {
                li { (line) }
            }
        }
    }
}

pub(crate) async fn handler_index(uri: Uri) -> Markup {
    let file = File::open(RESUME_FILEPATH).expect("Failed to read resume.md!");
    let reader = io::BufReader::new(file);

    // Collect all lines into a Vec first so we know the total count
    let mut file_lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap_or_else(|e| e.to_string()))
        .collect();

    // Insert your header at the start
    file_lines.insert(0, "Printing resume...".to_string());

    let mut body_lines: Vec<Markup> = Vec::new();
    let total = file_lines.len();

    for (i, line) in file_lines.iter().enumerate() {
        // Only true if it's the very last element
        let is_last = i == total - 1;
        body_lines.push(tmpl_terminal_command(
            line,
            TerminalOutputOptions::builder().is_last(is_last).build(),
        ));
    }

    let body = tmpl_page_body(body_lines);
    tmpl_global_chrome_wrapper(uri, body)
}

pub(crate) async fn handler_system_design(uri: Uri) -> Markup {
    let body = html! {
        ({
            tmpl_terminal_command(
                "System Design...",
                TerminalOutputOptions::builder().is_last(true).build(),
            )
        })
    };
    tmpl_global_chrome_wrapper(uri, body)
}
pub(crate) async fn handler_algorithms(uri: Uri) -> Markup {
    let body = html! {
        ({
            tmpl_terminal_command(
                "Algorithms...",
                TerminalOutputOptions::builder().is_last(true).build(),
            )
        })
    };
    tmpl_global_chrome_wrapper(uri, body)
}
