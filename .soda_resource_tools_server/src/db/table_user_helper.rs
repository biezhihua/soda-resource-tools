use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::api::entity::LoginParams;
use crate::db::models::{User, UserCreateOrUpdate};
use crate::db::schema::table_user;
use crate::utils;

/// 用户验证
pub(crate) fn authenticate(conn: &mut SqliteConnection, login_params: LoginParams) -> Option<User> {
    if let Some(user) = read_with_name(conn, &login_params.username) {
        if utils::verify_password(&login_params.password, &user.password) {
            return Some(user);
        }
    }
    return None;
}

/// 创建用户，如果成功则返回新增条目数，否则返回0。
/// https://stackoverflow.com/questions/47626047/execute-an-insert-or-update-using-diesel
/// https://docs.rs/diesel/2.1.1/diesel/query_builder/struct.InsertStatement.html#method.on_conflict
pub(crate) fn create(conn: &mut SqliteConnection, item: &UserCreateOrUpdate) -> usize {
    return diesel::insert_into(table_user::table)
        .values(item)
        .on_conflict_do_nothing()
        .execute(conn)
        .unwrap();
}

pub(crate) fn delete_all(conn: &mut SqliteConnection) -> usize {
    return diesel::delete(table_user::table)
        .filter(table_user::id.le(i32::MAX))
        .execute(conn)
        .unwrap();
}

pub(crate) fn delete(conn: &mut SqliteConnection, id: i32) -> usize {
    return diesel::delete(table_user::table)
        .filter(table_user::dsl::id.eq(id))
        .execute(conn)
        .unwrap();
}

pub(crate) fn update(conn: &mut SqliteConnection, user: UserCreateOrUpdate) -> usize {
    return diesel::update(table_user::dsl::table_user.filter(table_user::name.eq(user.name)))
        .set((
            table_user::password.eq(user.password),
            table_user::email.eq(user.email),
            table_user::avatar.eq(user.avatar),
        ))
        .execute(conn)
        .unwrap();
}

pub(crate) fn read(conn: &mut SqliteConnection, id: i32) -> Option<User> {
    return table_user::table
        .filter(table_user::dsl::id.eq(id))
        .first(conn)
        .optional()
        .unwrap();
}

pub(crate) fn read_with_name(conn: &mut SqliteConnection, name: &str) -> Option<User> {
    return table_user::table
        .filter(table_user::dsl::name.eq(name))
        .first(conn)
        .optional()
        .unwrap();
}

pub(crate) fn read_with_id(conn: &mut SqliteConnection, id: i32) -> Option<User> {
    return table_user::table
        .filter(table_user::dsl::id.eq(id))
        .first(conn)
        .optional()
        .unwrap();
}


pub(crate) fn list(conn: &mut SqliteConnection) -> Vec<User> {
    return table_user::table
        .select(User::as_select())
        .load(conn)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::env;

    use diesel::{Connection, SqliteConnection};
    use dotenvy::dotenv;

    use crate::db::models::UserCreateOrUpdate;
    use crate::db::table_user_helper::delete_all;

    fn create_new_user(connection: &mut SqliteConnection) -> usize {
        let user = UserCreateOrUpdate {
            name: "admin".to_string(),
            password: "password".to_string(),
            permission: "admin".to_string(),
            email: None,
            avatar: None,
        };
        let ret = super::create(connection, &user);
        ret
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_db() {
        let connection = &mut establish_connection();
        {
            delete_all(connection);
            let ret = create_new_user(connection);
            assert_eq!(ret, 1);
        }
        {
            delete_all(connection);
            let ret = create_new_user(connection);
            let ret = delete_all(connection);
            assert_eq!(ret, 1);
        }
        {
            delete_all(connection);
            create_new_user(connection);
            let ret = super::list(connection);
            let ret = super::delete(connection, ret.get(0).unwrap().id);
            assert_eq!(ret, 1);
        }
        {
            let connection = &mut establish_connection();
            delete_all(connection);
            create_new_user(connection);
            let ret = super::list(connection);
            let ret = super::read(connection, ret.get(0).unwrap().id);
            assert_eq!("admin", ret.unwrap().name);
        }

        {
            let connection = &mut establish_connection();
            delete_all(connection);
            create_new_user(connection);
            let ret = super::list(connection);
            assert_eq!(1, ret.len());
        }

        {
            let connection = &mut establish_connection();
            delete_all(connection);
            create_new_user(connection);
            let user = UserCreateOrUpdate {
                name: "admin".to_string(),
                password: "test".to_string(),
                permission: "admin".to_string(),
                email: None,
                avatar: None,
            };
            let ret = super::update(connection, user);
            let name = super::read_with_name(connection, "admin").unwrap();
            assert_eq!("test", name.password);
        }
    }

    fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
