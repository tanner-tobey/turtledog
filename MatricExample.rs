// docs/examples/matrix.bb
//
// BlueBird has built-in support for matrices, making it a powerful tool
// for numerical computing. The compiler checks matrix dimensions at compile
// time to prevent invalid operations.

fn main() {
    // Define a 2x3 matrix of 64-bit floating-point numbers.
    let matrix_a: Matrix<f64, 2, 3> = [[1.0, 2.0, 3.0],
                                       [4.0, 5.0, 6.0]]

    // Define a 3x1 matrix (a column vector).
    let matrix_b: Matrix<f64, 3, 1> = [[7.0],
                                       [8.0],
                                       [9.0]]

    print("Matrix A:")
    print(matrix_a)

    print("Matrix B:")
    print(matrix_b)

    // The '@' operator performs matrix multiplication.
    // The result will be a 2x1 matrix.
    let result = matrix_a @ matrix_b

    print("Result of A @ B:")
    print(result)

    // The following would cause a compile-time error because the inner
    // dimensions do not match (A has 3 columns, but A has 2 rows).
    // let invalid_result = matrix_a @ matrix_a;
}
