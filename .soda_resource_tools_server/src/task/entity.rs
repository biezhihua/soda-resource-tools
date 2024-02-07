use rocket::serde::json;
use rocket::serde::json::Value;

use crate::db::models::ManagementRule;

#[derive(Debug)]
pub(crate) struct TaskMsg {
    pub(crate) key: String,
    pub(crate) data: Value,
}

impl TaskMsg {

    pub fn task_management(rule: ManagementRule) -> TaskMsg {
        return TaskMsg {
            key: "task_management".to_string(),
            data: json::to_value(rule).unwrap(),
        };
    }
}

#[derive(Debug)]
pub(crate) enum TaskMsgKey {}
