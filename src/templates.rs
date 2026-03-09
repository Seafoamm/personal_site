use maud::{DOCTYPE, Markup, html};

fn tmpl_term_prompt(content: &str) -> Markup {
    html! {
        p { "$ " (content) }
    }
}

pub(crate) async fn tmpl_example_term_prompt() -> Markup {
    tmpl_term_prompt("Hello world!")
}

pub(crate) async fn tmpl_index() -> Markup {
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
                    p."term-window-title-right clear-spacing" {
                        "Terminal UI inspired resume/CV/portfolio/blog"
                    }
                }
                hr;
                nav."term-window-nav" {
                    ul."term-window-nav-tab-bar" {
                        a href="." { "About" }
                        a href="/system_design" { "System Design" }
                        a href="/algorithms" { "Algorithms" }
                    }
                }
                hr;

                "1. Intro\n
                2. Resume\n
                3. Projects Showcase"
            }
        }
    }
}
