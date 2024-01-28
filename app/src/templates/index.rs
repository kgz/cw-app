use std::collections::HashMap;

use actix_web::HttpRequest;
use actix_web::Result;
use maud::PreEscaped;
use maud::{html, Markup};

use crate::data::routes::Route;
use crate::Environments;
use crate::APP_ENV;

pub async fn index(req: HttpRequest, routes: HashMap<&'static str, Route>) -> Result<Markup> {
    let params = req.path().split("/").collect::<Vec<&str>>();
    //last
    let path = params[params.len() - 1];

    let is_debug = APP_ENV.env == Environments::DEV;

    let current_route = routes.get(path);

    let keywords = match current_route {
        Some(route) => format!(
            "fractal', 'attractor', 'react', 'threejs, {}, {}",
            route.description,
            path.replace("_", " ")
        ),
        None => "404".to_string(),
    };
    let imageUrl = match current_route {
        Some(route) => format!("https://matf.dev/icons/{}.png", path),
        None => "https://matf.dev/static/404.png".to_string(),
    };

    println!("current_route: {:?}", current_route);

    Ok(html!(
        html {
            head {
                title { "Hello" }
                @if is_debug {
                    script type="module" {
                    (PreEscaped("
                            console.log('Runnigng in dev mode')
                            window.dev = true;
                            import RefreshRuntime from 'https://localhost:3000/@react-refresh'
                            RefreshRuntime.injectIntoGlobalHook(window)
                            window.$RefreshReg$ = () => {}
                            window.$RefreshSig$ = () => (type) => type
                            window.__vite_plugin_react_preamble_installed__ = true;
                        "))
                    }
                    script defer type="module" src="https://localhost:3000/src/index.tsx" {}
                } @else {
                    script src="/static/index.min.js" {}
                }

                @if current_route.is_some() {
                    // basic meta tags
                    meta name="description" content=(current_route.unwrap().description) {}
                    meta name="keywords" content=(keywords) {}
                    meta name="author" content="Mat Frayne" {}
                    meta name="url" content="https://matf.dev" {}
                    meta name="og:title" content=(path.replace("_", " ")) {}
                    meta name="og:type" content="website" {}
                    meta name="og:url" content="https://matf.dev" {}
                    meta name="og:image" content=(imageUrl) {}
                    meta name="og:description" content=(current_route.unwrap().description) {}
                    meta name="og:site_name" content="matf.dev" {}
                    meta name="twitter:card" content="summary_large_image" {}
                    meta name="twitter:site" content="@matf_dev" {}
                    meta name="twitter:creator" content="@matf_dev" {}
                    meta name="twitter:title" content=(path.replace("_", " ")) {}
                    meta name="twitter:description" content=(current_route.unwrap().description) {}
                    meta name="twitter:image" content=(imageUrl) {}
                    meta name="twitter:image:alt" content=(current_route.unwrap().description) {}
                    meta name="twitter:domain" content="matf.dev" {}
                    meta name="twitter:label1" content="Written in" {}
                    meta name="twitter:data1" content="Rust" {}
                } @else {
                    meta name="description" content="404" {}
                }

            }
            body {
                // h1 { "Hello World!" }
                // p { "path: " (path) }
                // p { "is_debug: " (is_debug) }
                div id="root" {}
                div {
                    routes {
                        @for (key, value) in routes {
                            div {
                                h1 { (key) }
                                p { (value.description) }
                            }
                        }
                    }
                }
            }
        }
    ))
}
