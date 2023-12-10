use std::time::Duration;

use tokio::sync::mpsc::Sender;

use crate::db::{establish_db_connection, table_management_rule_helper};
use crate::task::entity::TaskMsg;

pub(crate) async fn start_task(tx: Sender<TaskMsg>) {
    // 间隔24小时触发
    let secs = 60 * 60 * 24;
    let mut interval_timer = tokio::time::interval(Duration::new(secs, 0));
    loop {
        interval_timer.tick().await;
        let sender = tx.clone();
        tokio::spawn(async move {
            do_task_resource_management(&sender).await;
        }); // For async task
    }
}

async fn do_task_resource_management(tx: &Sender<TaskMsg>) {
    tracing::info!("do_task_resource_management");

    let connection = &mut establish_db_connection();
    let list = table_management_rule_helper::list(connection);
    for rule in list {
        tx.try_send(TaskMsg::task_management(rule)).unwrap();
    }
}
