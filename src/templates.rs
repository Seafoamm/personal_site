use axum::http::Uri;
use maud::{DOCTYPE, Markup, html};

fn tmpl_terminal_line(content: &str) -> Markup {
    html! {
        span."terminal_prompt cursor-prompt" { "$ " (content) }
    }
}

pub(crate) async fn tmpl_example_term_prompt() -> Markup {
    tmpl_terminal_line("Hello world!")
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
                        }
                        p { "( Brandon Lu )" }
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
pub(crate) async fn handler_index(uri: Uri) -> Markup {
    let body = html! {
        (tmpl_terminal_line("About..."))
    };
    tmpl_global_chrome_wrapper(uri, body)
}

pub(crate) async fn handler_system_design(uri: Uri) -> Markup {
    let body = html! {
        (tmpl_terminal_line("System Design..."))
    };
    tmpl_global_chrome_wrapper(uri, body)
}

pub(crate) async fn handler_algorithms(uri: Uri) -> Markup {
    let body = html! {
        (tmpl_terminal_line("Algorithms..."))
    };
    tmpl_global_chrome_wrapper(uri, body)
}
