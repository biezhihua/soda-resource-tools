use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use soda_resource_tools_lib::soda::utils::token::Token;

use crate::{global, utils};
use crate::api::entity::{AccessTokenParams, CurrentGetResult, Empty, LoginParams, LoginResult, Response};
use crate::db::{RocketDb, table_user_helper};
use crate::db::models::{User, UserCreateOrUpdate};

#[get("/api/user/current/get")]
pub(crate) async fn api_user_current_get(db: RocketDb, token: Option<AccessTokenParams>) -> Json<Response<CurrentGetResult>> {
    tracing::info!("token = {:?}", token);

    if let Some(token) = token {
        if let Some(token) = utils::verification_token(token.token) {
            let id = token.key.parse::<i32>().unwrap();
            let user = db.run(move |conn| {
                return table_user_helper::read_with_id(conn, id);
            }).await;
            if let Some(user) = user {
                let result = CurrentGetResult {
                    name: user.name,
                    permission: user.permission,
                    avatar: user.avatar,
                };
                tracing::info!("result = {:?}", result);
                return Response::success_to_json(result);
            }
        } else {
            return Response::failure_code_msg_to_json("403", "token校验不通过");
        }
    }
    return Response::failure_code_msg_to_json("401", "请先登录");
}

#[post("/api/user/login/account", format = "application/json", data = "<params>")]
pub(crate) async fn api_user_login_account(db: RocketDb, params: Json<LoginParams>) -> Json<Response<LoginResult>> {
    let login_params = params.clone().into_inner();

    tracing::info!("login_params = {:?}", login_params);

    let user = db.run(|conn| {
        return table_user_helper::authenticate(conn, login_params);
    }).await;

    tracing::info!("user = {:?}", user);

    return match user {
        None => {
            let result = LoginResult {
                status: "error".to_string(),
                access_token: "".to_string(),
            };

            tracing::info!("result = {:?}", result);

            Response::success_to_json(result)
        }
        Some(user) => {
            let config = global::CONFIG.lock().unwrap();

            let token = utils::create_token(Token {
                key: user.id.to_string(),
                expire_time_millis: config.access_token_expire_millis,
            });
            tracing::info!("config = {:?}, token = {}", config, token);
            let result = LoginResult {
                status: "ok".to_string(),
                access_token: token,
            };

            tracing::info!("result = {:?}", result);

            Response::success_to_json(result)
        }
    };
}

#[post("/api/user/login/out")]
pub(crate) async fn api_user_login_out(token: Option<AccessTokenParams>) -> Json<Response<()>> {
    tracing::info!("token = {:?}", token);
    return match token {
        Some(_) => {
            Response::success_default_to_json()
        }
        None => {
            Response::failure_default_to_json()
        }
    };
}

#[post("/api/user/create", data = "<user>")]
pub(crate) async fn api_user_create(db: RocketDb, user: Json<UserCreateOrUpdate>) -> Json<Response<Empty>> {
    let item = user.into_inner();
    let ret = db.run(move |conn| {
        return table_user_helper::create(conn, &item);
    }).await;
    tracing::info!("ret = {}", ret);
    return Response::success_default_to_json();
}

#[get("/api/user/list")]
pub(crate) async fn api_user_list(db: RocketDb) -> Json<Response<Vec<User>>> {
    let ret = db.run(move |conn| {
        return crate::db::table_user_helper::list(conn);
    }).await;
    tracing::info!("ret = {:?}", ret);
    return Response::success_to_json(ret);
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("api user stage", |rocket| async {
        rocket.mount("/", routes![
            api_user_current_get,
            api_user_login_account,
            api_user_login_out,
            api_user_create,
            api_user_list,
        ])
    })
}
