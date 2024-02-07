use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::entity::{Empty, ManagementRuleParams, ManagementSettingResult, Response};
use crate::db::{RocketDb, table_management_rule_helper};
use crate::db::models::ManagementRule;
use crate::global;
use crate::task::entity::TaskMsg;
use crate::task::TaskState;

#[get("/api/resource/management/action?<id>")]
pub(crate) async fn api_resource_management_action(db: RocketDb, resource_tx: &State<TaskState>, id: String) -> Json<Response<Empty>> {
    return if let Some(raw_config_item) = table_management_rule_helper::read_with_id(db, id.parse().unwrap()).await {
        let task_msg = TaskMsg::task_management(raw_config_item);
        let sender = resource_tx.task_sender.clone();
        sender.try_send(task_msg).unwrap();

        Response::success_default_to_json()
    } else {
        Response::failure_code_msg_to_json("1000", &format!("Not fount data, id = {}", id))
    };
}

#[get("/api/resource/management/list")]
pub(crate) async fn api_resource_management_list(db: RocketDb) -> Json<Response<Vec<ManagementRuleParams>>> {
    let list: Vec<ManagementRule> = db.run(move |conn| {
        return table_management_rule_helper::list(conn);
    }).await;
    let result = ManagementRuleParams::convert_list(list);
    return Response::success_to_json(result);
}

#[get("/api/resource/management/delete/<id>")]
pub(crate) async fn api_resource_management_delete(db: RocketDb, id: i32) -> Json<Response<Empty>> {
    tracing::info!("{}", id);
    table_management_rule_helper::delete_with_id(db, id).await;
    return Response::success_default_to_json();
}

#[post("/api/resource/management/update", format = "application/json", data = "<item>")]
pub(crate) async fn api_resource_management_update(db: RocketDb, item: Json<ManagementRuleParams>) -> Json<Response<Empty>> {
    let rule = item.into_inner();

    tracing::info!("{:?}", rule);

    let ret = table_management_rule_helper::update_with_rule(db, rule).await;

    tracing::info!("ret = {}", ret);

    return Response::success_default_to_json();
}

#[post("/api/resource/management/create", format = "application/json", data = "<item>")]
pub(crate) async fn api_resource_management_create(db: RocketDb, item: Json<ManagementRuleParams>) -> Json<Response<Empty>> {
    let rule = item.into_inner();
    let item = ManagementRuleParams::create_from_web(&rule);

    tracing::info!("{:?}", item);

    let ret = table_management_rule_helper::create_with_rule(db, item).await;

    tracing::info!("ret = {}", ret);

    if rule.is_monitor_running() {
        // let handle = WatchEventHandler {
        //
        // };
        // soda::watcher::watch_path(rule.src.clone(), );
    }

    return Response::success_default_to_json();
}

#[get("/api/resource/setting/get")]
pub(crate) async fn api_resource_setting_get() -> Json<Response<ManagementSettingResult>> {
    let config = global::CONFIG.lock().unwrap();
    return Response::success_to_json(ManagementSettingResult {
        television_rename_format: config.tv_rename_format.clone(),
        film_rename_format: config.movie_rename_format.clone(),
    });
}

#[post("/api/resource/setting/update", format = "application/json", data = "<item>")]
pub(crate) async fn api_resource_setting_update(item: Json<ManagementSettingResult>) -> Json<Response<()>> {
    let setting = item.into_inner();

    let mut config = global::CONFIG.lock().unwrap();
    config.tv_rename_format = setting.television_rename_format.clone();
    config.movie_rename_format = setting.film_rename_format.clone();
    config.save();

    return Response::success_default_to_json();
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("api resource stage", |rocket| async {
        rocket.mount("/", routes![
            api_resource_management_action,
            api_resource_management_list,
            api_resource_management_create,
            api_resource_management_update,
            api_resource_management_delete,
            api_resource_setting_get,
            api_resource_setting_update,
        ])
    })
}
