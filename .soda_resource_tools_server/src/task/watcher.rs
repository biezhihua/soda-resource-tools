use std::path::PathBuf;

use soda_resource_tools_lib::soda;
use soda_resource_tools_lib::soda::watcher::FileWatchEventHandler;

use crate::db::{establish_db_connection, table_management_rule_helper};

pub(crate) async fn start_task() {
    tracing::info!("start");
    let connection = &mut establish_db_connection();
    let list = table_management_rule_helper::list(connection);
    for rule in list {

        // 启动新线程进行目录监控
        tokio::spawn(async move {
            tracing::info!("watch_path start");
            soda::watcher::watch_path(rule.src.clone(), FileWatcher {});
            tracing::info!("watch_path end")
        });
    }
    tracing::info!("end");
}

struct FileWatcher {}

impl FileWatchEventHandler for FileWatcher {
    fn handle_any_event(&mut self, paths: Vec<PathBuf>) {
        todo!()
    }

    fn handle_access_event(&mut self, paths: Vec<PathBuf>) {
        todo!()
    }

    fn handle_create_event(&mut self, paths: Vec<PathBuf>) {
        tracing::info!("{paths:?}");
    }

    fn handle_modify_event(&mut self, paths: Vec<PathBuf>) {
        todo!()
    }

    fn handle_remove_event(&mut self, paths: Vec<PathBuf>) {
        todo!()
    }

    fn handle_other_event(&mut self, paths: Vec<PathBuf>) {
        todo!()
    }
}