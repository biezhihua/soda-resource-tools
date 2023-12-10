pub(crate) fn now_time_format() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
