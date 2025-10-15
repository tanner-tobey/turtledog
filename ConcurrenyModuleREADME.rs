Concurrency ModuleThis module provides the asynchronous runtime and task scheduler for BlueBird.Design RationaleModern applications are heavily I/O-bound (e.g., waiting for network requests or disk reads). A traditional thread-per-task model is inefficient and does not scale well.BlueBird uses a more efficient asynchronous, multi-threaded scheduler based on a work-stealing algorithm, similar to modern runtimes like Tokio (Rust) and Goroutines (Go).How it works:A global thread pool is created with a small number of OS threads (typically one per CPU core).When a BlueBird program uses spawn, it creates a lightweight, asynchronous "task".These tasks are placed on a queue. Any available thread can pick up a task from the queue and execute it.If a task needs to wait for I/O (e.g., rx.recv()), it yields control, allowing the thread to execute other tasks. Once the I/O is ready, the task is placed back on the queue.This model allows a small number of threads to handle tens of thousands of concurrent tasks efficiently.Concurrency Model DiagramUsage ExampleBlueBird Code:spawn {
    // This block becomes an async task scheduled by the runtime.
    print("Running in parallel.")
}
Rust Implementation (Conceptual):// In bluebird_runtime/src/concurrency/scheduler.rs
use tokio::runtime::{Builder, Runtime};
use tokio::task;

/// Initializes the multi-threaded Tokio runtime.
pub fn init() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4) // Or num_cpus::get()
        .enable_all()
        .build()
        .unwrap()
}

/// The function called by BlueBird's `spawn` keyword.
pub fn spawn_task(future: impl std::future::Future + Send + 'static) {
    task::spawn(future);
}
