use rocket::fairing::AdHoc;
use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::api::entity::{ActionsParams, Response};
use crate::task::TaskState;

#[post("/api/actions", format = "application/json", data = "<params>")]
pub(crate) async fn api_actions(params: Json<ActionsParams>, resource_tx: &State<TaskState>) -> Json<Response<Value>> {
    let action = params.into_inner();
    tracing::info!("action = {:?}", action);

    // match action.action_type.as_str() {
    //     "file_recognize" => {
    //         if let Some(file_path) = action.file_path {
    //             return if let Some(metadata) = soda_resource_tools_lib::soda::recognize_movie_and_tv_metadata(&file_path) {
    //                 Response::success_to_json(json::to_value(metadata).unwrap())
    //             } else {
    //                 Response::failure_code_msg_to_json("1001", "文件识别失败")
    //             };
    //         }
    //     }
    //     _ => {}
    // }

    return Response::failure_default_to_json();
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("actions stage", |rocket| async {
        rocket.mount("/", routes![
            api_actions
        ])
    })
}