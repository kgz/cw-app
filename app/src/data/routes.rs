use std::collections::HashMap;

#[derive(Debug)]
pub struct Route {
    pub description: String,
}

pub fn gen_path(path: &str) -> String {
    format!("/{}", path.replace(" ", "").as_str().trim_end_matches("/")).to_string()
}

pub fn get_routes() -> HashMap<&'static str, Route> {
    let mut hash: HashMap<&str, Route> = HashMap::new();

    hash.insert(
        "bedhead_attractor",
        Route {
            description: "test description".to_string(),
        },
    );

    return hash;
}
