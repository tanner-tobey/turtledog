fn main() {
    let matrix_a: Matrix<f64, 2, 3> = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
    let matrix_b: Matrix<f64, 3, 1> = [[7.0], [8.0], [9.0]]
    let result = matrix_a @ matrix_b
    print(result)
}