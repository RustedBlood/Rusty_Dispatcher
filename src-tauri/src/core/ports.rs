use crate::core::domain::Process;
use crate::core::errors::CoreErrors;

pub trait ProcessMonitor {
    fn list_processes(&self) -> Result<Vec<Process>, CoreErrors>;
}
