use actix_web::HttpRequest;
use actix_web::Result;
use maud::{html, Markup, PreEscaped};
use crate::data;
pub async fn robots(req: HttpRequest) -> Result<Markup> {

    let routes = data::routes::get_routes();

    // generate a list of routes
    let mut routes_list = String::new();
    for (key, _) in routes.iter() {
        routes_list.push_str(&format!("Allow: {}\n", data::routes::gen_path(key)));
    }

    Ok(html! {
        (PreEscaped(format!("
            User-agent: *
            {}
            Sitemap: https://matf.dev/sitemap.xml
        ", routes_list)))
    })
}
