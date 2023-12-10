use diesel::{Insertable, Queryable, Selectable};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::db::schema::table_management_rule)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct ManagementRule {
    /// 主键
    pub(crate) id: i32,
    /// 源目录
    pub(crate) src: String,
    /// 整理目标目录
    pub(crate) target: String,
    /// 源目录内容类型
    pub(crate) content_type: String,
    /// 整理模式
    pub(crate) mode: String,
    /// 整理周期
    pub(crate) period: String,
    /// 运行状态
    pub(crate) status: String,
    /// 监控源目录变化
    pub(crate) monitor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::db::schema::table_management_rule)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct ManagementRuleCreate {
    pub(crate) src: String,
    pub(crate) target: String,
    pub(crate) content_type: String,
    pub(crate) mode: String,
    pub(crate) period: String,
    pub(crate) status: String,
    pub(crate) monitor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::db::schema::table_user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) permission: String,
    pub(crate) email: Option<String>,
    pub(crate) avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::db::schema::table_user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct UserCreateOrUpdate {
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) permission: String,
    pub(crate) email: Option<String>,
    pub(crate) avatar: Option<String>,
}