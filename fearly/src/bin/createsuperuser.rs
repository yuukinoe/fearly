use juli_orm_core::{DatabaseConfig, initialize_db_with_config};
#[allow(unused_imports)]
use std::io::{self, *};
use fearly::models::{CreateUser, User};
use fearly::load_config;
use fearly::utils::hash_password;
use slugify::slugify;
#[allow(unused_imports)]
use chrono::NaiveDateTime;

#[allow(unused_imports)]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    println!("--- Create Superuser ---");
    print!("Username: ");
    stdout().flush().unwrap();
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Password: ");
    stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();
    let password = password.trim();

    if password.is_empty() {
        println!("Are you sure you want to create a superuser with an empty password?");
        println!("Type 'yes' to confirm overwrite password creation:");
        stdout().flush().unwrap();
        let mut confirmation = String::new();
        stdin().read_line(&mut confirmation).unwrap();
        let confirmation = confirmation.trim();
        if confirmation != "yes" && confirmation != "y" {
            println!("Operation cancelled");
            return;
        }
    } else if password.len() < 8 {
        println!("Normally password should be at least 8 characters long confirming overwrite password creation.");
        println!("Type 'yes' to confirm overwrite password creation:");
        stdout().flush().unwrap();
        let mut confirmation = String::new();
        stdin().read_line(&mut confirmation).unwrap();
        let confirmation = confirmation.trim();
        if confirmation != "yes" && confirmation != "y" {
            println!("Operation cancelled");
            return;
        }
    }

    let config = match load_config("../config.yml") {
        Ok(config) => {
            println!("Config loaded successfully");
            config
        }
        Err(e) => {
            println!("Error loading config: {}", e);
            panic!("Failed to load config");
        }
    };

    let _ = initialize_db_with_config(DatabaseConfig {
        host: config.database.host,
        port: config.database.port,
        username: config.database.user,
        password: config.database.password,
        namespace: config.database.namespace,
        database: config.database.database,
    })
    .await;

    let hashed = hash_password(password.as_bytes());

    match User::query().where_name_eq(username.to_string()).fetch().await {
      Ok(user) => {
          if !user.is_empty() {
              println!("User already exists");
              return;
          }
      }
      Err(e) => {
        println!("Error fetching user: {}", e);
        panic!("Failed to fetch user");
      }
    };


    let created = User::manage()
        .create(CreateUser {
            name: username.to_string(),
            password: hashed,
            avatar: None,
            email: None,
            description: None,
            published: true,
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            is_superuser: true,
            is_staff: true,
            is_active: true,
            is_banned: false,
            slug: slugify!(format!("{}", username.to_string()).as_str()).to_string(),
        })
        .await;
    println!("Superuser '{}' created!", created.unwrap().name);
}
