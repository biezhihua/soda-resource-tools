use rocket::fairing::AdHoc;
use rocket::serde::json::{json, Json, Value};

use crate::api::entity::UpdateBasicSettingParams;

#[post("/api/basic/setting/update", format = "application/json", data = "<params>")]
pub(crate) async fn api_basic_setting_update(params: Json<UpdateBasicSettingParams>) -> Value {
    json!({"success": true, "data": {}})
}

#[get("/api/basic/setting/get")]
pub(crate) async fn api_basic_setting_get() -> Value {
    json!(
            {
                "success": false,
                "error_code": "401",
                "error_message": "请先登录！",
                "data": {
                }
            }
        )
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("api setting stage", |rocket| async {
        rocket.mount("/", routes![
            api_basic_setting_update,
            api_basic_setting_get
        ])
    })
}