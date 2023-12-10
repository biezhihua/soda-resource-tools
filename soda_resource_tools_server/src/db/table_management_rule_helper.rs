use rocket_sync_db_pools::diesel;

use crate::api::entity::ManagementRuleParams;
use crate::db::models::{ManagementRule, ManagementRuleCreate};
use crate::db::RocketDb;
use crate::db::schema::table_management_rule;

use self::diesel::prelude::*;

pub(crate) async fn read_with_id(db: RocketDb, id: i32) -> Option<ManagementRule> {
    tracing::info!("{:?}", id);

    let result: Option<ManagementRule> = db.run(move |conn| {
        table_management_rule::table
            .filter(table_management_rule::dsl::id.eq(id))
            .first(conn)
            .optional()
    }).await.unwrap();
    return result;
}

pub(crate) fn list(conn: &mut SqliteConnection) -> Vec<ManagementRule> {
    return table_management_rule::table
        .select(ManagementRule::as_select())
        .load(conn)
        .unwrap();
}

pub(crate) async fn read_with_rule(db: &RocketDb, item: &ManagementRuleParams) -> Option<ManagementRule> {
    tracing::info!("{:?}", item);

    let src_value = item.src.clone();
    let target_value = item.target.clone();
    let result: Option<ManagementRule> = db.run(move |conn| {
        table_management_rule::table
            .filter(table_management_rule::src.eq(src_value))
            .filter(table_management_rule::target.eq(target_value))
            .first(conn)
            .optional()
    }).await.unwrap();
    return result;
}


pub(crate) async fn create_with_rule(db: RocketDb, item: ManagementRuleCreate) -> usize {
    return db.run(move |conn| {
        return create(conn, &item);
    }).await;
}

pub(crate) fn create(conn: &mut SqliteConnection, item: &ManagementRuleCreate) -> usize {
    return diesel::insert_into(table_management_rule::table)
        .values(item)
        .on_conflict_do_nothing()
        .execute(conn)
        .unwrap();
}

pub(crate) async fn delete_with_id(db: RocketDb, id: i32) {
    tracing::info!("{:?}", id);
    db.run(move |conn| {
        diesel::delete(table_management_rule::table)
            .filter(table_management_rule::dsl::id.eq(id))
            .execute(conn)
    }).await.unwrap();
}

pub(crate) async fn update_with_rule(db: RocketDb, rule: ManagementRuleParams) -> usize {
    return db.run(move |conn| {
        return update(conn, rule);
    }).await;
}

pub(crate) fn update(conn: &mut SqliteConnection, item: ManagementRuleParams) -> usize {
    return diesel::update(table_management_rule::dsl::table_management_rule.filter(table_management_rule::dsl::id.eq(item.id)))
        .set((
            table_management_rule::dsl::src.eq(item.src),
            table_management_rule::dsl::target.eq(item.target),
            table_management_rule::dsl::content_type.eq(item.content_type),
            table_management_rule::dsl::mode.eq(item.mode),
            table_management_rule::dsl::period.eq(item.period),
            table_management_rule::dsl::status.eq(item.status),
            table_management_rule::dsl::monitor.eq(item.monitor),
        ))
        .execute(conn)
        .unwrap();
}