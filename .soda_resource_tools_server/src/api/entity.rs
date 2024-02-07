use hmac::digest::KeyInit;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, json, Serialize};
use rocket::serde::json::{Json, Value};

use crate::db::models::{ManagementRule, ManagementRuleCreate};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Empty {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    ///
    pub success: bool,

    ///
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", rename = "data")]
    pub data: Option<T>,

    //
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<String>,

    //
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", rename = "errorMessage")]
    pub error_message: Option<String>,

    /// enum ErrorShowType {
    ///   SILENT = 0,
    ///   WARN_MESSAGE = 1,
    ///   ERROR_MESSAGE = 2,
    ///   NOTIFICATION = 3,
    ///   REDIRECT = 9,
    // }
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", rename = "showType")]
    pub show_type: Option<i32>,
}

impl<T> Response<T> {
    pub fn failure_code_msg_to_json(error_code: &str, error_message: &str) -> Json<Response<T>> {
        return Json(Response {
            success: false,
            data: None,
            error_code: Some(error_code.to_string()),
            error_message: Some(error_message.to_string()),
            show_type: Some(1),
        });
    }

    pub fn success_to_json(data: T) -> Json<Response<T>> {
        return Json(Response {
            success: true,
            data: Some(data),
            error_code: None,
            error_message: None,
            show_type: None,
        });
    }

    pub fn success_default_to_json() -> Json<Response<T>> {
        return Json(Response {
            success: true,
            data: None,
            error_code: None,
            error_message: None,
            show_type: None,
        });
    }

    pub fn failure_default_to_json() -> Json<Response<T>> {
        return Json(Response {
            success: false,
            data: None,
            error_code: None,
            error_message: None,
            show_type: Some(0),
        });
    }

    pub fn success(data: T) -> Self {
        return Response {
            success: true,
            data: Some(data),
            error_code: None,
            error_message: None,
            show_type: None,
        };
    }

    pub fn success_default() -> Self {
        return Response {
            success: true,
            data: None,
            error_code: None,
            error_message: None,
            show_type: None,
        };
    }

    pub fn failure_default() -> Self {
        return Response {
            success: false,
            data: None,
            error_code: None,
            error_message: None,
            show_type: Some(0),
        };
    }
    pub fn failure_msg(msg: String) -> Self {
        return Response {
            success: false,
            data: None,
            error_code: None,
            error_message: Some(msg),
            show_type: Some(1),
        };
    }

    pub fn failure_code_msg(error_code: &str, error_message: &str) -> Self {
        return Response {
            success: false,
            data: None,
            error_code: Some(error_code.to_string()),
            error_message: Some(error_message.to_string()),
            show_type: Some(1),
        };
    }
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        return Response {
            success: true,
            data: None,
            error_code: None,
            error_message: None,
            show_type: None,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UpdateBasicSettingParams {
    log_output_level: String,
    log_output_type: String,
    web_password: String,
    web_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ActionsParams {
    #[serde(rename = "type")]
    pub(crate) action_type: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "file_path")]
    pub(crate) file_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ManagementSettingResult {
    pub(crate) television_rename_format: String,
    pub(crate) film_rename_format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ManagementRuleParams {
    pub(crate) id: i32,
    pub(crate) src: String,
    pub(crate) target: String,
    pub(crate) content_type: String,
    pub(crate) mode: String,
    pub(crate) period: String,
    pub(crate) status: String,
    pub(crate) monitor: String,
}

impl ManagementRuleParams {
    pub(crate) fn is_status_running(&self) -> bool {
        return self.status == "running";
    }

    pub(crate) fn is_monitor_running(&self) -> bool {
        return self.monitor == "running";
    }

    pub(crate) fn is_stop(&self) -> bool {
        return self.status == "stop";
    }
}

impl ManagementRule {
    pub(crate) fn convert_to_params(self) -> ManagementRuleParams {
        return ManagementRuleParams::create_from_db(self);
    }
}

impl ManagementRuleParams {
    pub(crate) fn create_from_json(src: Value) -> ManagementRuleParams {
        let sync_item: ManagementRuleParams = json::from_value(src).unwrap();
        return sync_item;
    }

    pub(crate) fn create_from_db(src: ManagementRule) -> ManagementRuleParams {
        return ManagementRuleParams {
            id: src.id.clone(),
            src: src.src.clone(),
            target: src.target.clone(),
            content_type: src.content_type.clone(),
            mode: src.mode.clone(),
            period: src.period.clone(),
            status: src.status.clone(),
            monitor: src.monitor.clone(),
        };
    }

    pub(crate) fn create_from_web(src: &ManagementRuleParams) -> ManagementRuleCreate {
        return ManagementRuleCreate {
            src: src.src.clone(),
            target: src.target.clone(),
            content_type: src.content_type.clone(),
            mode: src.mode.clone(),
            period: src.period.clone(),
            status: src.status.clone(),
            monitor: src.monitor.clone(),
        };
    }

    pub(crate) fn convert_list(src: Vec<ManagementRule>) -> Vec<ManagementRuleParams> {
        let mut result = Vec::new();
        for src_item in src {
            result.push(ManagementRuleParams::create_from_db(src_item))
        }
        return result;
    }
}

#[derive(Debug)]
pub struct AccessTokenParams {
    pub(crate) token: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AccessTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AccessTokenParams {
    type Error = AccessTokenError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let access_token = request.headers().get_one("access_token");
        match access_token {
            Some(token) => {
                let access_token = token.to_string();
                Outcome::Success(AccessTokenParams {
                    token: access_token
                })
            }
            None => {
                Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginParams {
    #[serde(rename = "username")]
    pub(crate) username: String,

    #[serde(rename = "password")]
    pub(crate) password: String,

    #[serde(rename = "auto_login")]
    #[serde(default)]
    pub(crate) auto_login: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginResult {
    #[serde(rename = "status")]
    pub(crate) status: String,

    #[serde(rename = "access_token")]
    pub(crate) access_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct CurrentGetResult {
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "permission")]
    pub(crate) permission: String,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", rename = "avatar")]
    pub(crate) avatar: Option<String>,
}