#[macro_use]
extern crate rocket;
use crate::views::*;
use fearly::*;
use juli_orm_core::{DatabaseConfig, initialize_db_with_config};
use rocket::shield::{Frame, Shield};

#[launch]
async fn rocket() -> _ {
    let c_config = match load_config("../config.yml") {
        Ok(config) => {
            println!("Config loaded successfully");
            config
        }
        Err(e) => {
            println!("Error loading config: {}", e);
            panic!("Failed to load config");
        }
    };
    let db_config = DatabaseConfig {
        host: c_config.database.host,
        port: c_config.database.port,
        username: c_config.database.user,
        password: c_config.database.password,
        namespace: c_config.database.namespace,
        database: c_config.database.database,
    };
    initialize_db_with_config(db_config).await.unwrap();

    let shield = Shield::default().disable::<Frame>();
    rocket::build()
        .attach(CORS)
        .attach(shield)
        .mount(
            String::from("/") + &format!("{}", CONFIG.directories.media).replace("/", ""),
            rocket::fs::FileServer::from(format!("{}", CONFIG.directories.media).replace("/", "")),
        )
        .mount("/", routes![
            login,
            is_logged_in_get_bool,
            logout,
            is_logged_in_get,
            add_fear,
            delete_fear,
            edit_fear,
            get_fears,
        ])
}
