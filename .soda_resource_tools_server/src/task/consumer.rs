use std::time::SystemTime;

use tokio::sync::mpsc::Receiver;

use soda_resource_tools_lib::soda;

use crate::api::entity::ManagementRuleParams;
use crate::global;
use crate::task::entity::TaskMsg;

pub(crate) async fn start_task(mut rx: Receiver<TaskMsg>) {
    loop {
        if let Some(msg) = rx.recv().await {
            tracing::info!("receive new msg key = {:?}", msg.key);
            let start_time = SystemTime::now();
            tracing::info!("task start time = {:?}", start_time);
            if "task_management" == msg.key {
                let rule = ManagementRuleParams::create_from_json(msg.data);
                do_task_resource_management(rule)
            }
            let end_time = SystemTime::now();
            tracing::info!("task end time = {:?}", (end_time.duration_since(start_time)).unwrap());
        }
    }
}

fn do_task_resource_management(rule: ManagementRuleParams) {
    tracing::info!("{:?}", rule);
    if rule.is_status_running() {
        let server_config = global::CONFIG.lock().unwrap();

        //
        // let mut params = soda::resource::Params::new();
        // params.src_directory = rule.src.clone();
        // params.target_directory = rule.target.clone();
        // params.resource_type = rule.content_type;
        //

        // soda::management(params);
    }
}
