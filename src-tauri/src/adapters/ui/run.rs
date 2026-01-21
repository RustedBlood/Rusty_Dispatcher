use crate::adapters::ui::handlers;
use crate::core::ports::ProcessMonitor;

pub fn run(core_repo: impl ProcessMonitor + 'static + Send + Sync) {
    tauri::Builder::default()
        .manage(core_repo)
        .invoke_handler(tauri::generate_handler![handlers::get_processes_list])
        .run(tauri::generate_context!())
        .expect("Failed to build UI correctly")
}
