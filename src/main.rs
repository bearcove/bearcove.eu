use dioxus::prelude::*;

const STYLES: Asset = asset!("./main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" }
        document::Link { rel: "icon", r#type: "image/jxl", href: "/favicon.jxl" }
        document::Link { rel: "stylesheet", href: "{STYLES}" }
        div { class: "container",
            // Header
            header { class: "header",
                div { class: "logo-section",
                    h1 { class: "logo", "bearcove" }
                    p { class: "tagline", "Teaching Rust, making videos, maintaining open-source software" }
                }
            }

            // Main content
            main { class: "main-content",
                // Hero section
                section { class: "hero",
                    p { class: "hero-text",
                        strong { "bearcove" }
                        " was founded by "
                        a { href: "https://fasterthanli.me", target: "_blank", "@fasterthanlime" }
                        " in late 2024 to keep doing what they do best: teach Rust, make videos, and maintain open-source software."
                    }

                    div { class: "video-container",
                        iframe {
                            width: "100%",
                            height: "315",
                            src: "https://www.youtube.com/embed/zo6yZisg7N0",
                            title: "YouTube video player",
                            frame_border: "0",
                            allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                            allowfullscreen: true
                        }
                    }
                }

                // Open Source section
                section { class: "section",
                    h2 { "Open source" }

                    p { "bearcove maintains big Rust projects:" }
                    ul { class: "project-list",
                        li {
                            a { href: "https://facet.rs", target: "_blank", strong { "facet" } }
                            " — a take on reflection in Rust, allows a ton of nice use cases"
                        }
                        li {
                            a { href: "https://home.bearcove.eu", target: "_blank", strong { "home" } }
                            " — the CMS that powers this website and several others"
                        }
                    }

                    p { "Some smaller crates:" }
                    ul { class: "project-list",
                        li {
                            a { href: "https://github.com/bearcove/rubicon", target: "_blank", strong { "rubicon" } }
                            " — macros to make dangerous dynamic linking pattern in Rust a little safer"
                        }
                        li {
                            a { href: "https://github.com/bearcove/dylo", target: "_blank", strong { "dylo" } }
                            " — enables \"impl\" and \"consumer\" crates when doing dynamic linking"
                        }
                        li {
                            a { href: "https://github.com/bearcove/rc-zip", target: "_blank", strong { "rc-zip" } }
                            " — a sans-io ZIP implementation in Rust focused on compatibility"
                        }
                        li {
                            a { href: "https://github.com/fasterthanlime/timelord", target: "_blank", strong { "timelord" } }
                            " — restores timestamps between builds to let cargo and Rust build scripts see that the build is clean"
                        }
                    }

                    p { "The following are not actively being maintained:" }
                    ul { class: "project-list inactive",
                        li {
                            a { href: "https://github.com/bearcove/loona", target: "_blank", strong { "loona" } }
                            " — an HTTP/1+2 implementation on top of io_uring, with kTLS support (on hold)"
                        }
                        li {
                            a { href: "https://github.com/bearcove/merde", target: "_blank", strong { "merde" } }
                            " — another take on serialization with declarative macros and \"metastack\" support (superseded by facet)"
                        }
                    }
                }

                // Content Creation section
                section { class: "section",
                    h2 { "Content creation" }

                    p { "bearcove operates:" }
                    ul { class: "content-list",
                        li {
                            a { href: "https://fasterthanli.me", target: "_blank", "fasterthanli.me" }
                            " with long-form Rust content"
                        }
                        li {
                            a { href: "https://youtube.com/@fasterthanlime", target: "_blank", "YouTube @fasterthanlime" }
                            " with a variety of tech videos"
                        }
                    }

                    p { "That content receives funding through:" }
                    ul { class: "content-list",
                        li {
                            a { href: "https://github.com/sponsors/fasterthanlime", target: "_blank", "GitHub Sponsors" }
                        }
                        li {
                            a { href: "https://patreon.com/fasterthanlime", target: "_blank", "Patreon" }
                        }
                    }

                    p {
                        "Additionally, bearcove cooperates with "
                        a { href: "https://onevariable.com", target: "_blank", "OneVariable UG" }
                        " to produce:"
                    }
                    ul { class: "content-list",
                        li {
                            "the "
                            a { href: "https://sdr-podcast.com", target: "_blank", "Self-Directed Research" }
                            " podcast"
                        }
                    }
                }

                // Legal section
                section { class: "section legal",
                    h2 { "Legal / Mentions légales" }

                    p {
                        strong { "bearcove SARL" }
                        " is a company with a capital of 10'000€, registered with the RCS (registre du commerce et des sociétés) of Lyon under the number "
                        strong { "934 149 618" }
                        "."
                    }

                    p { "Its address is:" }
                    blockquote { class: "address",
                        "bearcove SARL"
                        br {}
                        "4 Quai Jean Moulin"
                        br {}
                        "c/o La Cordée SAS"
                        br {}
                        "69001 Lyon"
                        br {}
                        "France"
                    }

                    p {
                        "And its VAT number is "
                        strong { "FR41 934 149 618" }
                        " (see the symmetry?)."
                    }

                    p {
                        "The owner and manager of bearcove is Amos Wenger, you can reach them at "
                        a { href: "mailto:admin@bearcove.eu", "admin@bearcove.eu" }
                        "."
                    }

                    p {
                        "This website and other bearcove properties like "
                        a { href: "https://fasterthanli.me", target: "_blank", "fasterthanli.me" }
                        " are hosted on "
                        a { href: "https://hetzner.com", target: "_blank", "Hetzner" }
                        ", whereas DNS is handled by "
                        a { href: "https://gcore.com", target: "_blank", "gcore" }
                        "."
                    }
                }
            }

            // Footer
            footer { class: "footer",
                p { "© 2024 bearcove SARL. All rights reserved." }
            }
        }
    }
}
