use actix_web::HttpRequest;
use actix_web::Result;
use maud::{html, Markup, PreEscaped};
use crate::data;
pub async fn sitemap(req: HttpRequest) -> Result<Markup> {

    let routes = data::routes::get_routes();

    // generate a list of routes for sitemap.xml
    let mut routes_list = String::new();
    for (key, _) in routes.iter() {
        routes_list.push_str(&format!("{}\n", data::routes::gen_path(key)));
    }


    Ok(html! {
        (PreEscaped(format!("
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>
            <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">
            {}
            </urlset>
        ", routes_list)))
    })

    

    
}
