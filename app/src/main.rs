use std::{fs::File, io::BufReader, path::PathBuf};

use actix_cors::Cors;

use actix_files::{self as fs, NamedFile};
use actix_web::{http, web, App, HttpRequest, HttpServer, Result};
use fake::{Dummy, Fake, Faker};
use rand::Rng;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::ser::SerializeStruct;
pub mod templates {
    automod::dir!(pub "./src/templates");
}
pub mod data {
    automod::dir!(pub "./src/data");
}

#[derive(PartialEq, Debug)]
pub enum Environments {
    DEV,
    TEST,
    PROD,
}

impl std::fmt::Display for Environments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Environments::DEV => write!(f, "dev"),
            Environments::TEST => write!(f, "test"),
            Environments::PROD => write!(f, "prod"),
        }
    }
}

const IS_DEBUG: bool = !!cfg!(debug_assertions);


#[derive(Debug)]
pub struct Env<'a> {
    pub env: Environments,
    pub auto_login_id: &'a str,
}

pub const APP_ENV: Env = Env {
    env: match IS_DEBUG {
        true => Environments::DEV,
        false => Environments::PROD,
        _ => panic!("IS_DEBUG is not bool"),
    },
    auto_login_id: "1",
};



// async fn index(_: HttpRequest) -> Result<fs::NamedFile> {
//     Ok(NamedFile::open("static/index.html")?)
// }

async fn static_media(req: HttpRequest) -> Result<fs::NamedFile> {
    let file = req.match_info().get("file").unwrap();
    let path = format!("static/{}", file);
    Ok(NamedFile::open(path)?)
}

/**
 * 
 */
async fn icons(req: HttpRequest) -> Result<fs::NamedFile> {
    let file = req.match_info().get("file").unwrap();
    // make safe
    let file_safe = file.replace("..", "");
    let path = format!("src/data/images/{}", file_safe);
    println!("{:?}", path);
    Ok(NamedFile::open(path)?)
}

struct User {
    id: String,
    name: String,
    email: String,
    phone: String,
    address: String,
    company: String,
}

impl Dummy<Faker> for User {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        User {
            id: Faker.fake(),
            name: Faker.fake(),
            email: Faker.fake(),
            phone: Faker.fake(),
            address: Faker.fake(),
            company: Faker.fake(),
        }
    }

    fn dummy(_: &Faker) -> Self {
        User {
            id: Faker.fake(),
            name: Faker.fake(),
            email: Faker.fake(),
            phone: Faker.fake(),
            address: Faker.fake(),
            company: Faker.fake(),
        }
    }

    // to json
}

impl serde::ser::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("phone", &self.phone)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("company", &self.company)?;
        state.end()
    }
}

async fn dummy_data_as_json(_: HttpRequest) -> Result<String> {
    let mut users = Vec::new();
    for _ in 0..0 {
        users.push(User::dummy(&Faker));
    }
    let json = serde_json::to_string(&users).unwrap();
    Ok(json)
}

fn load_certs(cert: PathBuf, key: PathBuf) -> Result<ServerConfig, String> {
    let cert_file = &mut BufReader::new(File::open(&cert).map_err(|e| e.to_string())?);
    let key_file = &mut BufReader::new(File::open(&key).map_err(|e| e.to_string())?);

    let cert_chain = certs(cert_file)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        return Err("Could not locate PKCS 8 private keys.".to_string());
    }

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    config
        .with_single_cert(cert_chain, keys.remove(0))
        .map_err(|e| e.to_string())
}

async fn version(_: HttpRequest) -> Result<String> {
    Ok("0.0.1".to_string())
}

pub const SCOPE: &str = match APP_ENV.env {
    Environments::DEV => "/chaos",
    Environments::PROD => "",
    Environments::TEST => "/",
    _ => panic!("Could not start server"),
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App is running in {} mode", APP_ENV.env);

    let cert_file = std::env::current_dir().unwrap().join("localhost.pem");
    let key_file = std::env::current_dir().unwrap().join("localhost-key.pem");

    // set arg default value

    let cert_config = load_certs(cert_file, key_file).unwrap();

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://localhost")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors)
        
        .route("/static/{file:.*}", web::get().to(static_media))
        .service(
            web::scope(SCOPE)
            // .route("/", web::get().to(index))
                .route("/icons/{file:.*}", web::get().to(icons))
                .route("/version", web::get().to(version))
                .route(
                    "",
                    web::get().to(|req: HttpRequest| {
                        templates::index::index(req, data::routes::get_routes())
                    }),
                )
                .route(
                    "/",
                    web::get().to(|req: HttpRequest| {
                        templates::index::index(req, data::routes::get_routes())
                    }),
                )
                .route(
                    "/{path}",
                    web::get().to(|req: HttpRequest| {
                        templates::index::index(req, data::routes::get_routes())
                    }),
                )
                // static media
                // dummy data
                .route(
                    "/{tail:.*}",
                    web::get().to(templates::fourofour::four_o_four),
                )
                .route("/static/{file:.*}", web::get().to(static_media))
                .route("/icons/{file:.*}", web::get().to(icons))
                .route(
                    "/{tail:.*}",
                    web::get().to(templates::fourofour::four_o_four),
                ),
        )
        
        // add localhost-key.pem and localhost.pem to root
        // .service(fs::Files::new("/", "./static/").index_file("index.html"))
    });
    // .bind("0.0.0.0:2020")?;

    let server = match APP_ENV.env {
        Environments::DEV => server.bind_rustls("0.0.0.0:2020", cert_config)?,
        Environments::PROD => server.bind("0.0.0.0:2020")?,
        Environments::TEST => todo!(),
        _ => panic!("Could not start server"),
    };

    // print server url
    println!("Server running at https://localhost:2020/");
    server.run().await
}
