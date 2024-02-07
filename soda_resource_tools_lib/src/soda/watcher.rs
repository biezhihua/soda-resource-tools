use std::path::{Path, PathBuf};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub trait FileWatchEventHandler: Send + 'static {
    fn handle_any_event(&mut self, paths: Vec<PathBuf>);
    fn handle_access_event(&mut self, paths: Vec<PathBuf>);
    fn handle_create_event(&mut self, paths: Vec<PathBuf>);
    fn handle_modify_event(&mut self, paths: Vec<PathBuf>);
    fn handle_remove_event(&mut self, paths: Vec<PathBuf>);
    fn handle_other_event(&mut self, paths: Vec<PathBuf>);
}

pub fn watch_path<F>(target: String, mut handler: F)
where
    F: FileWatchEventHandler,
{
    let (tx, rx) = std::sync::mpsc::channel();
    if let Ok(mut watcher) = RecommendedWatcher::new(tx, Config::default()) {
        if let Ok(_) = watcher.watch(Path::new(&target.replace("/", "\\")), RecursiveMode::Recursive) {
            for res in rx {
                match res {
                    Ok(event) => match event.kind {
                        EventKind::Any => {
                            handler.handle_any_event(event.paths);
                        }
                        EventKind::Access(_) => {
                            handler.handle_access_event(event.paths);
                        }
                        EventKind::Create(_) => {
                            handler.handle_create_event(event.paths);
                        }
                        EventKind::Modify(_) => {
                            handler.handle_create_event(event.paths);
                        }
                        EventKind::Remove(_) => {
                            handler.handle_remove_event(event.paths);
                        }
                        EventKind::Other => {
                            handler.handle_other_event(event.paths);
                        }
                    },
                    Err(error) => tracing::debug!("Error: {error:?}"),
                }
            }
        }
    }
}
