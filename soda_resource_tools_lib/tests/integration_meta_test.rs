#![allow(warnings)]

#[cfg(test)]
mod meta_tests {
    use std::fs::File;
    use std::io::Read;

    use serde_json::Value;
    use tracing::Level;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;

    use soda_resource_tools_lib::soda;

    #[test]
    fn test_movie_and_tv_metadata() {
        init_tracing();
        test_parse("tests/movie_and_tv_metadata.json");
    }

    /// 初始化日志配置
    fn init_tracing() {
        // Configure a `tracing` subscriber that logs traces emitted by the server.
        tracing_subscriber::fmt()
            // Filter what traces are displayed based on the RUST_LOG environment
            // variable.
            //
            // Traces emitted by the example code will always be displayed. You
            // can set `RUST_LOG=tokio=trace` to enable additional traces emitted by
            // Tokio itself.
            .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
            // Log events when `tracing` spans are created, entered, exited, or
            // closed. When Tokio's internal tracing support is enabled (as
            // described above), this can be used to track the lifecycle of spawned
            // tasks on the Tokio runtime.
            .with_span_events(FmtSpan::FULL)
            // Set this subscriber as the default, to collect all traces emitted by
            // the program.
            .init();
    }

    fn test_parse(path: &str) {
        // Open the file
        let mut file = File::open(path).expect("Failed to open the file");

        // Read the file content into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read the file");

        // Parse the JSON string into a serde_json::Value
        let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

        // Check if the parsed JSON is an array
        if let Some(items) = json["items"].as_array() {
            items.iter().rev().for_each(|element| {
                let title = element["title"].as_str().unwrap();

                if !element["metadata"].is_null() {
                    let metadata = soda::create_mt_metadata(title).unwrap();

                    if let Some(year) = element["metadata"]["year"].as_str() {
                        if !year.is_empty() {
                            assert_eq!(year, metadata.year.unwrap());
                        }
                    }

                    if let Some(season) = element["metadata"]["season"].as_str() {
                        assert_eq!(season, metadata.season);
                    }

                    if let Some(episode) = element["metadata"]["episode"].as_str() {
                        assert_eq!(episode, metadata.episode);
                    }

                    if let Some(resolution) = element["metadata"]["resolution"].as_str() {
                        assert_eq!(resolution, metadata.resolution);
                    }

                    if let Some(source) = element["metadata"]["source"].as_str() {
                        assert_eq!(source, metadata.source);
                    }

                    if let Some(video_codec) = element["metadata"]["video_codec"].as_str() {
                        assert_eq!(video_codec, metadata.video_codec);
                    }

                    if let Some(container) = element["metadata"]["container"].as_str() {
                        assert_eq!(container, metadata.extension);
                    }

                    if let Some(release_group) = element["metadata"]["release_group"].as_str() {
                        assert_eq!(release_group, metadata.release_group);
                    }

                    if let Some(cn_name) = element["metadata"]["cn_name"].as_str() {
                        assert_eq!(cn_name, metadata.title_cn);
                    }

                    if let Some(en_name) = element["metadata"]["en_name"].as_str() {
                        assert_eq!(en_name, metadata.title_en);
                    }

                    if let Some(audio_codec) = element["metadata"]["audio_codec"].as_str() {
                        assert_eq!(audio_codec, metadata.audio_codec);
                    }
                }
            });
        }
    }
}
