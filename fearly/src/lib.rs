use once_cell::sync::Lazy;
use serde_yaml;
use std::fs;
pub mod config;
pub mod models;
use crate::config::Config;
pub mod utils;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::options;
use rocket::{Request, Response};

pub struct CORS;
pub mod serializers;
pub mod views;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // Allow both local and tunnel origins
        let origin = req.headers().get_one("Origin");
        let v_port = CONFIG.routers.web[0].port.clone();
        let port = if v_port != 80 && v_port != 443 {
            format!(":{}", v_port)
        } else {
            String::new()
        };
        let allowed_origin = if let Some(origin) = origin {
            if origin.contains("trycloudflare.com") {
                origin.to_string()
            } else {
                format!(
                    "{}",
                    CONFIG.routers.web[0].protocol.clone()
                        + "://"
                        + &CONFIG.routers.web[0].hostname.clone()
                        + &port
                )
            }
        } else {
            format!(
                "{}",
                CONFIG.routers.web[0].protocol.clone()
                    + "://"
                    + &CONFIG.routers.web[0].hostname.clone()
                    + &port
            )
        };
        // dbg!(&allowed_origin);
        res.set_header(Header::new("Access-Control-Allow-Origin", allowed_origin));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, UPDATE, DELETE",
        ));
        // res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization, Origin, Accept, X-Requested-With",
        ));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        // res.set_header(Header::new(
        //     "Access-Control-Allow-Headers",
        //     "Content-Type, Authorization",
        // ));
    }
}

#[options("/<_..>")]
pub fn cors_options() -> rocket::http::Status {
    rocket::http::Status::Ok
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| load_config("../config.yml").expect("Failed to load config"));
