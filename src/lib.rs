#[macro_use]
extern crate old_log;

pub fn test_fn() {
    log::info!("Newer log info macro");
    info!("Older log info macro");
}
