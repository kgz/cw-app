use actix_web::HttpRequest;
use actix_web::Result;
use maud::PreEscaped;
use maud::{html, Markup};

pub async fn four_o_four(req: HttpRequest) -> Result<Markup> {
    Ok(html!(
        html {

            body {
                h1 { "404" }
                p { "path: " (req.path()) }
                p { "is_debug: " (true) }
                div id="root" {}
            }
        }
    ))
}
