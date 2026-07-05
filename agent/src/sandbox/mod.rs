pub trait SandboxOrchestrator {
    fn setup(&self, target_file: &str) -> Result<(), String>;
    fn execute(&self) -> Result<(), String>;
    fn cleanup(&self) -> Result<(), String>;
}

pub mod qemu;
pub use qemu::QemuSandbox;
