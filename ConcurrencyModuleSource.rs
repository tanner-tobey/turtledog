//! # The BlueBird Concurrency Runtime
//!
//! This module provides the async task scheduler. It's built on top of
//! Tokio to provide a world-class, battle-tested foundation.

use tokio::runtime::{Builder, Runtime};
use std::future::Future;

/// The global runtime instance.
/// In a real program, this would be managed more carefully,
/// possibly with `lazy_static` or `once_cell`.
static mut RUNTIME: Option<Runtime> = None;

/// Initializes the scheduler. The compiler ensures this is called once.
pub fn init_scheduler() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create BlueBird runtime.");

    unsafe {
        RUNTIME = Some(runtime);
    }
}

/// The function that the BlueBird `spawn` keyword compiles down to.
/// It takes a future and runs it on the global runtime.
pub fn bluebird_spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    if let Some(rt) = unsafe { RUNTIME.as_ref() } {
        rt.spawn(future);
    }
}
