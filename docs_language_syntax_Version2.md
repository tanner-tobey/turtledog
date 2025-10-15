# BlueBird Syntax Guide

## Variables
let x = 5         // Immutable
let mut y = 10    // Mutable

## Functions
fn add(a: int, b: int) -> int {
    return a + b
}

## Control Flow
if x > 5 {
    print("x is large")
} else {
    print("x is small")
}

## Matrix Ops
let m: Matrix<f64, 2, 2> = [[1,2],[3,4]]
let r = m1 @ m2