use crate::core::{domain::Process, errors::CoreErrors, ports::ProcessMonitor};

#[tauri::command]
pub fn get_processes_list(
    core_repo: tauri::State<'_, Box<dyn ProcessMonitor + 'static + Send + Sync>>,
) -> Result<Vec<Process>, CoreErrors> {
    core_repo.list_processes()
}
