use maud::{Markup, html};

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
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link href="/output.css" rel="stylesheet";
                link type="image/x-icon" href="./favicon.png" rel="icon";
                script src="https://unpkg.com/hyperscript.org@0.9.14" {}
            }
            header."bg-surface border border-surface shadow-sm mb-4 font-primary text-primary flex-row flex justify-between" {
                div."ml-2 flex-1 text-left py-0" {
                    p."text-xs" {
                        "Placeholder for icons/CTA"
                    }
                }
                div."flex-1 flex-col py-2 text-center" {
                    a href="." {
                        h1."py-4 text-primary text-4xl flex-1 justify-evenly bg-surface font-primary text-center" {
                            "Lu::Brandon"
                        }
                    }
                    p {
                        "( Brandon Lu )"
                    }
                }
                p."text-xs mr-2 flex-1 text-right py-0" {
                    "Terminal UI inspired resume/CV/portfolio/blog"
                }
            }
            nav."border border-surface shadow-sm flex-col flex gap-2 py-4 bg-surface" {
                ul."justify-evenly flex gap-2 width-full" {
                    a."text-accent" href="." {
                        "About"
                    }
                    a."text-accent" href="/system_design" {
                        "System Design"
                    }
                    a."text-accent" href="/algorithms" {
                        "Algorithms"
                    }
                }
                hr;
                ul."[&_img]:border [&_img]:border-surface [&_img]:p-2 [&_img]:size-12 flex flex-row gap-2 justify-center" {
                    a href="https://www.youtube.com/@DoubleColon11" {
                        img src="./yt_icon.png";
                    }
                    a href="https://linktr.ee/doublecolon11" {
                        img src="./linktree_icon.webp";
                    }
                }
            }
            body."bg-main text-frost" {
                "1. Intro
  2. Resume
  3. Projects Showcase"
            }
        }
    }
}
