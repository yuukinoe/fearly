use crate::CONFIG;
use crate::models::{CreateFear, CreateSessions, Fear, Sessions, User
};
use crate::serializers::*;
use crate::utils::{self, errorln};
use chrono::{Local, NaiveDateTime, Utc};
use juli_orm_core::ForeignKey;
use rocket::data::ByteUnit;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{delete, get, patch, post};
use url::form_urlencoded;
use surrealdb::sql::Thing;
extern crate slugify;

pub async fn update_session_id(user: User, session_id: String) -> Sessions {
    use chrono::{Duration, Local};
    let local_time_now = Local::now().naive_local();
    let sessions = Sessions::query()
        .where_user_id_eq(user.id.clone())
        .where_expiration_lte(local_time_now)
        .fetch()
        .await;
    if let Ok(sessions) = sessions {
        for session in sessions {
            Sessions::manage()
                .delete(session.id.id.to_raw())
                .await
                .unwrap_or_else(|e| {
                    errorln(format!("Error deleting session: {}", e));
                });
        }
    }

    let expiration_date = local_time_now + Duration::minutes(CONFIG.session_duration.clone());
    let new_session = Sessions::manage()
        .create(CreateSessions {
            user_id: user.id.clone(),
            session_id: session_id.clone(),
            expiration: expiration_date,
        })
        .await
        .unwrap_or_else(|e| {
            errorln(format!("Error creating session: {}", e));
            panic!("Failed to create session");
        });
    new_session
}

pub fn get_session_id_expiration(session: Sessions) -> Option<(String, NaiveDateTime)> {
    Some((session.session_id.clone(), session.expiration))
}

#[post("/login", data = "<login>")]
pub async fn login(jar: &CookieJar<'_>, login: Json<UserLoginData>) -> (Status, Json<Response>) {
    let user = match User::query()
        .where_name_eq(login.username.clone())
        .fetch()
        .await
    {
        Ok(users) => {
            if users.is_empty() {
                return (
                    Status::from_code(401).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message(
                            "Invalid username or password".to_string(),
                        ),
                        status: 401,
                    }),
                );
            } else {
                users.into_iter().next().unwrap()
            }
        }
        Err(e) => {
            errorln(format!("Error querying users: {}", e));
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Internal server error".to_string()),
                    status: 500,
                }),
            );
        }
    };

    if !utils::password_verifier(login.password.clone().as_bytes(), user.clone().password) {
        return (
            Status::from_code(401).unwrap(),
            Json(Response {
                message: GenericResponse::Message("Invalid username or password".to_string()),
                status: 401,
            }),
        );
    }
    use rocket::http::SameSite;
    use uuid::Uuid;
    // Generate a new session_id and update expiration
    let session_id = Uuid::new_v4().to_string();
    let user = update_session_id(user, session_id.clone()).await;

    // Set cookie with expiration
    if let Some((_, expiration)) = get_session_id_expiration(user) {
        let mut cookie = Cookie::new("session_id", session_id);
        cookie.set_expires(
            rocket::time::OffsetDateTime::from_unix_timestamp(expiration.and_utc().timestamp())
                .unwrap_or_else(|_| rocket::time::OffsetDateTime::now_utc()),
        );
        cookie.set_path("/");
        if CONFIG.debug.cookies.clone() {
            cookie.set_same_site(SameSite::Lax);
            println!("Session with expiration date");
        } else {
            cookie.set_same_site(SameSite::None);
        };
        cookie.set_secure(!CONFIG.debug.cookies.clone());
        cookie.set_http_only(!CONFIG.debug.cookies.clone());

        jar.add(cookie);
    } else {
        let mut cookie = Cookie::new("session_id", session_id);
        cookie.set_path("/");
        jar.add(cookie);
    }
    if CONFIG.debug.prints.clone() {
        println!("success");
    }
    (
        Status::from_code(200).unwrap(),
        Json(Response {
            message: GenericResponse::Message("Login successful".to_string()),
            status: 200,
        }),
    )
}


pub async fn get_user_by_cookie(jar: &CookieJar<'_>) -> Result<User, String> {
    if CONFIG.debug.prints.clone() {
        println!("Getting user by cookie");
    }
    if let Some(cookie) = jar.get("session_id") {
        let session_id = cookie.value();
        if session_id == "" || session_id == "undefined" {
            return Err("User not found in database".to_string());
        }
        match Sessions::query()
            .where_session_id_eq(session_id.to_string())
            .fetch()
            .await
        {
            Ok(session) => {
                if !session.is_empty() {
                    let session = session.into_iter().next().unwrap();
                    if session.expiration > Local::now().naive_local() {
                        return Ok(
                            User::query()
                                .where_id_eq(session.user_id)
                                .fetch()
                                .await
                                .unwrap()
                                .into_iter()
                                .next()
                                .unwrap(),
                        );
                    }
                }
            }
            Err(e) => {
                if CONFIG.debug.prints.clone() {
                    errorln(format!("Error checking if user is logged in2: {}", e));
                }
            }
        }
    }
    Err("User not logged in".to_string())
}


pub async fn is_logged_in(jar: &CookieJar<'_>) -> Result<(User, bool), String> {
    if CONFIG.debug.prints.clone() {
        println!("Checking if user is logged in");
    }
    if let Some(cookie) = jar.get("session_id") {
        let session_id = cookie.value();
        if session_id == "" || session_id == "undefined" {
            return Err("User not logged in".to_string());
        }
        match Sessions::query()
            .where_session_id_eq(session_id.to_string())
            .fetch()
            .await
        {
            Ok(session) => {
                if !session.is_empty() {
                    let session = session.into_iter().next().unwrap();
                    if session.expiration > Local::now().naive_local() {
                        return Ok((
                            User::query()
                                .where_id_eq(session.user_id)
                                .fetch()
                                .await
                                .unwrap()
                                .into_iter()
                                .next()
                                .unwrap(),
                            true,
                        ));
                    }
                }
            }
            Err(e) => {
                if CONFIG.debug.prints.clone() {
                    errorln(format!("Error checking if user is logged in2: {}", e));
                }
            }
        }
    }
    Err("User not logged in".to_string())
}

#[get("/logged_in")]
pub async fn is_logged_in_get(jar: &CookieJar<'_>) -> (Status, Json<Response>) {
    // dbg!(&jar);
    // let a = is_logged_in(jar).await.unwrap()
    match is_logged_in(jar).await {
        Ok(logged_in) => {
            if logged_in.1 {
                if logged_in.0.is_staff && logged_in.0.is_active && !logged_in.0.is_banned {
                    let val = serde_json::to_value(&logged_in.0).unwrap_or_else(|e| {
                        errorln(format!("Error serializing user: {}", e));
                        serde_json::Value::Null
                    });
                    return (
                        Status::from_code(200).unwrap(),
                        Json(Response {
                            message: GenericResponse::Data(val),
                            status: 200,
                        }),
                    );
                }
            }
            return (
                Status::from_code(401).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Not logged in".to_string()),
                    status: 401,
                }),
            );
        }
        Err(e) => {
            if CONFIG.debug.prints.clone() {
                errorln(format!("Error checking if user is logged in1: {}", e));
            }
            return (
                Status::from_code(401).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Not logged in".to_string()),
                    status: 401,
                }),
            );
        }
    }
}


#[get("/logged_in_bool")]
pub async fn is_logged_in_get_bool(jar: &CookieJar<'_>) -> Json<bool> {

    match is_logged_in(jar).await {
        Ok(logged_in) => {
            if logged_in.1 {
                if logged_in.0.is_staff && logged_in.0.is_active && !logged_in.0.is_banned {

                    return Json(true);
                }
            }
            return Json(false);
        }
        Err(e) => {
            if CONFIG.debug.prints.clone() {
                errorln(format!("Error checking if user is logged in1: {}", e));
            }
            return Json(false);
        }
    }
}



#[post("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Json<bool> {
    jar.remove(Cookie::from("session_id"));
    Json(true)
}


#[post("/add_fear", data = "<data>")]
pub async fn add_fear(jar:&CookieJar<'_>, data: rocket::data::Data<'_>) -> (Status, Json<Response>) {
    let user = get_user_by_cookie(jar).await;
    if let Err(err) = user.clone() {
        return (
            Status::Unauthorized,
            Json(Response {
                status: 401,
                message: GenericResponse::Message(format!("Unauthorized: {}", err)),
            }),
        )
    }

    let mut body = String::new();
    data.open(ByteUnit::Kilobyte(4))
        .read_to_string(&mut body)
        .await
        .unwrap();
    let parsed = form_urlencoded::parse(body.as_bytes());

    let mut title: String = String::new();
    let mut content: String = String::new();
    let mut date: String = String::new();
    let mut time: String = String::new();
    let mut emotion: i8 = 0;
    let mut reaction: String = String::new();

    for (key, value) in parsed {
        match key.as_ref() {
            "title" => title = value.into_owned(),
            "content" => content = value.into_owned(),
            "emotion" => emotion = value.parse().unwrap_or(0),
            "reaction" => reaction = value.into_owned(),
            "date" => date = value.into_owned(),
            "time" => time = value.into_owned(),
            _ => {}
        }
    };
    let dt = NaiveDateTime::parse_from_str(
        &format!("{} {}", date, time),
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();



    let fear = CreateFear {
        user: ForeignKey::new(user.unwrap().id),
        title,
        content: content,
        reaction: reaction,
        emotion: emotion,
        datetime: Some(dt),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    match Fear::manage().create(fear).await {
        Ok(fear) => {
            (Status::Ok,
            Json(Response {
                status: 200,
                message: GenericResponse::FearData(fear),
            }))
        },
        Err(err) => {
            errorln(format!("Error creating fear: {}", err));
            (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Internal Server Error".to_string()),
                }),
            )
        }
    }

}


#[patch("/edit_fear", data = "<data>")]
pub async fn edit_fear(jar:&CookieJar<'_>, data: rocket::data::Data<'_>) -> (Status, Json<Response>) {
    let user = get_user_by_cookie(jar).await;
    if let Err(err) = user.clone() {
        return (
            Status::Unauthorized,
            Json(Response {
                status: 401,
                message: GenericResponse::Message(format!("Unauthorized: {}", err)),
            }),
        )
    }

    let mut body = String::new();
    data.open(ByteUnit::Kilobyte(4))
        .read_to_string(&mut body)
        .await
        .unwrap();
    let parsed = form_urlencoded::parse(body.as_bytes());

    let mut id: String = String::new();
    let mut title: String = String::new();
    let mut content: String = String::new();
    let mut date: String = String::new();
    let mut time: String = String::new();
    let mut emotion: i8 = 0;
    let mut reaction: String = String::new();


    for (key, value) in parsed {
        match key.as_ref() {
            "id" => id = value.into_owned(),
            "title" => title = value.into_owned(),
            "content" => content = value.into_owned(),
            "emotion" => emotion = value.parse().unwrap_or(0),
            "reaction" => reaction = value.into_owned(),
            "date" => date = value.into_owned(),
            "time" => time = value.into_owned(),
            _ => {}
        }
    };

    let dt = NaiveDateTime::parse_from_str(
        &format!("{} {}", date, time),
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();



    let fear = EditFear {
        title: title,
        content: content,
        emotion: emotion,
        reaction: reaction,
        datetime: Some(dt),
        updated_at: Some(Utc::now().naive_utc()),
    };

    if id.is_empty() {
        return (
            Status::InternalServerError,
            Json(Response {
                status: 500,
                message: GenericResponse::Message("Internal Server Error".to_string()),
            }),
        )
    }


    match Fear::manage().merge(id, fear).await {
        Ok(fear) => {
            (Status::Ok,
            Json(Response {
                status: 200,
                message: GenericResponse::FearData(fear),
            }))
        },
        Err(err) => {
            errorln(format!("Error creating fear: {}", err));
            (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Internal Server Error".to_string()),
                }),
            )
        }
    }
}


#[delete("/delete_fear/<id>")]
pub async fn delete_fear(jar:&CookieJar<'_>, id: &str) -> (Status, Json<Response>) {
    if id.is_empty() {
        return (
            Status::InternalServerError,
            Json(Response {
                status: 500,
                message: GenericResponse::Message("Internal Server Error".to_string()),
            }),
        )
    }
    let user = get_user_by_cookie(jar).await;

    let fear = Fear::query().where_id_eq(Thing::from(("fear", id))).fetch().await;

    match user {
        Ok(usr) =>{
            match fear {
                Ok(f) => {
                    if f.is_empty() {
                        return (
                            Status::NotFound,
                            Json(Response {
                                status: 404,
                                message: GenericResponse::Message("Fear not found".to_string()),
                            }),
                        )
                    } else {
                        if usr.id != f.into_iter().next().unwrap().user.data().unwrap().id {
                            return (
                                Status::Unauthorized,
                                Json(Response {
                                    status: 404,
                                    message: GenericResponse::Message("Unauthorized".to_string()),
                                }),
                            )
                        }
                    }
                },
                Err(e) => {
                    errorln(format!("Error getting fear: {}", e));
                    return (
                        Status::InternalServerError,
                        Json(Response {
                            status: 500,
                            message: GenericResponse::Message("Internal Server Error".to_string()),
                        }),
                    )
                }
            }

        },
        Err(err) => {
            errorln(format!("Error getting user: {}", err));
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Internal Server Error".to_string()),
                }),
            )
        }
    }


    let _ = Fear::manage().delete(id.to_string()).await;

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::Message("Fear deleted successfully".to_string()),
        })
    )

}


#[get("/get_fears")]
pub async fn get_fears(jar:&CookieJar<'_>) -> (Status, Json<Response>) {
    let user = get_user_by_cookie(jar).await;
    if let Err(_) = user.clone() {
        return (
            Status::Unauthorized,
            Json(Response {
                status: 401,
                message: GenericResponse::VecData(vec![]),
            }),
        )
    }

    match Fear::query().where_user_eq(ForeignKey::new(user.unwrap().id)).order_by("datetime", "desc").fetch_to_value().await {
        Ok(fears) => {
            (Status::Ok,
            Json(Response {
                status: 200,
                message: GenericResponse::VecData(fears),
            }))
        },
        Err(err) => {
            errorln(format!("Error getting fears: {}", err));
            (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Internal Server Error".to_string()),
                }),
            )
        }
    }
}
