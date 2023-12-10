use std::time::Duration;

use rocket::fairing::AdHoc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

use crate::task::entity::TaskMsg;

pub(crate) mod consumer;

pub(crate) mod schedule;

pub(crate) mod entity;
pub(crate) mod watcher;

pub(crate) struct TaskState {
    pub(crate) task_sender: Sender<TaskMsg>,
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("task stage", |rocket| async {
        let (task_sender, task_receiver) = mpsc::channel(128);
        let sender = task_sender.clone();

        // // 启动调度任务
        // tokio::spawn(async move {
        //     sleep(Duration::new(10, 0)).await;
        //     tracing::info!("schedule task start");
        //     schedule::start_task(sender).await;
        //     tracing::info!("schedule task stop")
        // });
        //
        // 启动消费者任务
        tokio::spawn(async move {
            sleep(Duration::new(5, 0)).await;
            tracing::info!("consumer task start");
            consumer::start_task(task_receiver).await;
            tracing::info!("consumer task stop")
        });
        //
        // // 启动监控任务
        // tokio::spawn(async move {
        //     sleep(Duration::new(5, 0)).await;
        //     tracing::info!("watcher task start");
        //     watcher::start_task().await;
        //     tracing::info!("watcher task stop")
        // });

        rocket
            .manage(TaskState { task_sender })
    })
}