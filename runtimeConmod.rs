//! BlueBird Concurrency Runtime Module
//! Last updated: 2025-10-15

use tokio::runtime::{Builder, Runtime};
use std::future::Future;
use std::sync::Once;

static INIT: Once = Once::new();
static mut RUNTIME: Option<Runtime> = None;

/// Initialize the global runtime
pub fn init() {
    INIT.call_once(|| {
        let rt = Builder::new_multi_thread()
            .worker_threads(num_cpus::get())
            .enable_all()
            .build()
            .expect("Failed to create runtime");
        
        unsafe {
            RUNTIME = Some(rt);
        }
    });
}

/// Spawn a new task
pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    unsafe {
        if let Some(rt) = RUNTIME.as_ref() {
            rt.spawn(future);
        }
    }
}