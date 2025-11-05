use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" }
        document::Link { rel: "icon", r#type: "image/jxl", href: "/favicon.jxl" }
        style { {CSS} }
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

const CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    line-height: 1.6;
    color: #333;
    background-color: #fafafa;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 20px;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

.header {
    padding: 60px 0 40px;
    text-align: center;
    border-bottom: 1px solid #e1e5e9;
    margin-bottom: 40px;
}

.logo {
    font-size: 3rem;
    font-weight: 700;
    color: #2d3748;
    margin-bottom: 8px;
    letter-spacing: -0.025em;
}

.tagline {
    font-size: 1.1rem;
    color: #718096;
    font-weight: 400;
}

.main-content {
    flex: 1;
}

.hero {
    margin-bottom: 60px;
}

.hero-text {
    font-size: 1.2rem;
    line-height: 1.7;
    margin-bottom: 40px;
    color: #4a5568;
}

.hero-text strong {
    color: #2d3748;
    font-weight: 600;
}

.video-container {
    position: relative;
    width: 100%;
    height: 0;
    padding-bottom: 56.25%; /* 16:9 aspect ratio */
    margin-bottom: 20px;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
}

.video-container iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: none;
}

.section {
    margin-bottom: 50px;
}

.section h2 {
    font-size: 2rem;
    font-weight: 600;
    color: #2d3748;
    margin-bottom: 20px;
    border-bottom: 2px solid #e2e8f0;
    padding-bottom: 8px;
}

.section p {
    margin-bottom: 16px;
    color: #4a5568;
    font-size: 1.05rem;
}

.project-list, .content-list {
    list-style: none;
    margin: 20px 0;
}

.project-list li, .content-list li {
    margin-bottom: 12px;
    padding-left: 20px;
    position: relative;
    color: #4a5568;
}

.project-list li::before, .content-list li::before {
    content: "•";
    color: #718096;
    position: absolute;
    left: 0;
    font-weight: bold;
}

.project-list.inactive li {
    opacity: 0.7;
}

.project-list.inactive li::before {
    content: "○";
}

a {
    color: #3182ce;
    text-decoration: none;
    transition: color 0.2s ease;
}

a:hover {
    color: #2c5282;
    text-decoration: underline;
}

.legal {
    background-color: #f7fafc;
    padding: 30px;
    border-radius: 8px;
    border-left: 4px solid #e2e8f0;
}

.address {
    background-color: #edf2f7;
    padding: 20px;
    border-left: 4px solid #cbd5e0;
    font-style: italic;
    color: #4a5568;
    margin: 20px 0;
    line-height: 1.8;
}

.footer {
    margin-top: 60px;
    padding: 30px 0;
    border-top: 1px solid #e1e5e9;
    text-align: center;
    color: #718096;
    font-size: 0.9rem;
}

/* Responsive design */
@media (max-width: 768px) {
    .container {
        padding: 0 16px;
    }

    .logo {
        font-size: 2.5rem;
    }

    .header {
        padding: 40px 0 30px;
    }

    .section h2 {
        font-size: 1.75rem;
    }

    .hero-text {
        font-size: 1.1rem;
    }

    .legal {
        padding: 20px;
    }
}

@media (max-width: 480px) {
    .logo {
        font-size: 2rem;
    }

    .tagline {
        font-size: 1rem;
    }

    .hero-text {
        font-size: 1rem;
    }

    .section h2 {
        font-size: 1.5rem;
    }
}
"#;
