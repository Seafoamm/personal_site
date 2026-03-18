use axum::http::Uri;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use std::fs::File;
use std::io::{self, BufRead};
use typed_builder::TypedBuilder;

const RESUME_FILEPATH: &str = "./assets/resume.md";
const TERMINAL_PROMPT: &str = "$ ";
const TERMINAL_DEFAULT_FONT_WEIGHT: i32 = 400;
const TERMINAL_DEFAULT_OPACITY: i32 = 1;
const TERMINAL_OUTPUT_CSS_CLASS_PREFIX: &str = "md_";

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

impl TerminalOutputOptions {
    pub fn get_terminal_output_option_css_classes(&self) -> String {
        // Map field references to their intended CSS name
        let flags = [
            (self.is_italic, "italic"),
            (self.is_bolded, "bolded"),
            (self.is_banner, "banner"),
            (self.is_command, "command"),
            (self.is_pill, "pill"),
            (self.is_uri, "uri"),
            (self.is_list, "list"),
            (self.is_last, "last"),
        ];

        let mut css_classes = flags
            .iter()
            .filter(|(is_active, _)| *is_active) // Only take true values
            .map(|(_, name)| name.with_md_css_prefix()) // Apply your trait method
            .collect::<Vec<_>>();

        css_classes.push(format!("{}{}", "weight-".with_md_css_prefix(), self.weight));
        css_classes.push(format!(
            "{}{}",
            "opacity-".with_md_css_prefix(),
            self.weight
        ));
        css_classes.join(" ")
    }
}

trait MarkdownCssPrefix {
    fn with_md_css_prefix(&self) -> String;
}

impl MarkdownCssPrefix for str {
    fn with_md_css_prefix(&self) -> String {
        format!("{}{}", TERMINAL_OUTPUT_CSS_CLASS_PREFIX, self)
    }
}

fn tmpl_terminal_command(content: &str, options: TerminalOutputOptions) -> Markup {
    // TODO: implement css classes
    let md_parser = Parser::new(content);
    let mut css_classes: Vec<String> = Vec::new();
    for event in md_parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => match level {
                HeadingLevel::H1 => css_classes.push("h1".with_md_css_prefix()),
                HeadingLevel::H2 => css_classes.push("h2".with_md_css_prefix()),
                HeadingLevel::H3 => css_classes.push("h3".with_md_css_prefix()),
                HeadingLevel::H4 => css_classes.push("h4".with_md_css_prefix()),
                HeadingLevel::H5 => css_classes.push("h5".with_md_css_prefix()),
                HeadingLevel::H6 => css_classes.push("h6".with_md_css_prefix()),
            },
            Event::Start(Tag::List(_)) => css_classes.push("list".with_md_css_prefix()),
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => css_classes.push("link".with_md_css_prefix()),
            _ => (),
        }
    }

    css_classes.push(options.get_terminal_output_option_css_classes());

    html! {
        span.terminal_prompt.(css_classes.join(" ")) { (TERMINAL_PROMPT) (content) }
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
