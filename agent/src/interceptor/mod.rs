pub trait ExecutionInterceptor {
    fn start(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}

pub mod macos;
pub use macos::MacOSEndpointSecurityInterceptor as NativeInterceptor;
