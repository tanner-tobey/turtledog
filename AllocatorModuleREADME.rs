Allocator ModuleThis module manages heap memory for BlueBird programs.Design RationaleBlueBird guarantees memory safety without a garbage collector (GC). Like Rust, it uses an ownership model where memory is automatically deallocated when a variable goes out of scope. This approach offers several key advantages:Predictable Performance: No GC pauses mean that application latency is consistent and predictable, which is critical for systems programming and APIs.Lower Memory Overhead: No need for a complex GC runtime, reducing the memory footprint of every BlueBird executable.Deterministic Deallocation: Resources are freed as soon as they are no longer needed, preventing memory leaks and making resource management simpler.The role of this allocator is to provide a fast and efficient backend for heap allocations (e.g., when a list is created or a string is resized). We may use a high-performance allocator like jemalloc to outperform standard system allocators.Memory Flow DiagramThis diagram shows what happens when a BlueBird list is created, used, and then goes out of scope.Usage ExampleBlueBird Code:fn create_list() {
    // 1. Runtime's allocator is called to allocate space on the heap for 10 integers.
    let my_list: list<int> = [0; 10]

    // 2. Use the list...

} // 3. 'my_list' goes out of scope here. The runtime immediately calls the deallocator
  //    to free the heap memory. No GC is involved.
Rust Implementation (Conceptual):// In bluebird_runtime/src/allocator/mod.rs

use std::alloc::{GlobalAlloc, Layout, System};

// A custom allocator could be defined here. For now, we use the system default.
pub struct BluebirdAllocator;

unsafe impl GlobalAlloc for BluebirdAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}
