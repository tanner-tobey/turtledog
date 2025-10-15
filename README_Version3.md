# BlueBird Programming Language

BlueBird is a high-performance, safe, and expressive programming language inspired by Python and Rust. Features include memory safety (ownership model), async concurrency, matrix operations, and fast APIs.

## Features

- Ownership-based memory management (no garbage collector)
- Concurrency: async runtime, lightweight tasks, channels
- Matrix operations for scientific computing
- Fast, declarative APIs

## Installation

**Prerequisites**
- Rust toolchain (1.70+)
- LLVM 14.0+
- CMake 3.20+

**Build**
```sh
git clone https://github.com/tanner-tobey/turtledog.git
cd turtledog
cargo build --workspace
```

## Usage

**Compile a BlueBird source file**
```sh
cargo run --bin bluebirdc -- examples/hello_world.bb
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT (see [LICENSE](LICENSE))