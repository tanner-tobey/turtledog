Matrix Computation EngineThis module provides the low-level implementation for BlueBird's matrix operations.Design RationaleNumerical computing performance is dominated by how efficiently data can be processed in bulk. Modern CPUs include SIMD (Single Instruction, Multiple Data) instruction sets (like SSE, AVX) that can perform a mathematical operation on multiple pieces of data (e.g., eight 32-bit floats) simultaneously.This engine is designed to:Store matrix data in a contiguous memory layout (column-major or row-major) to ensure cache-friendliness.Use SIMD intrinsics to perform operations like matrix multiplication, addition, and dot products in parallel at the hardware level.By making this a core part of the runtime, BlueBird can vastly outperform languages like Python that rely on interpreted, non-vectorized loops for similar tasks (unless using a C-extension like NumPy).Usage ExampleBlueBird Code:# The '@' operator is a syntax sugar that the compiler
# translates into a call to the runtime's matrix engine.
let result = matrix_a @ matrix_b
Rust Implementation (Conceptual):// In bluebird_runtime/src/matrix/mod.rs

#[repr(C)]
pub struct BluebirdMatrix<T, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

// The compiler generates a call to this function for the '@' operator.
pub fn matrix_multiply_f64<const R1: usize, const C1: usize, const C2: usize>(
    a: &BluebirdMatrix<f64, R1, C1>,
    b: &BluebirdMatrix<f64, C1, C2>,
) -> BluebirdMatrix<f64, R1, C2> {
    // Highly optimized implementation using SIMD intrinsics would go here.
    // ...
    unimplemented!("SIMD matrix multiplication logic goes here");
}
