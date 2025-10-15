# BlueBird Programming Language

BlueBird is a high-performance, safe, and expressive programming language inspired by Python and Rust. It features:

- Ownership-based memory management (no garbage collector)
- Concurrency with async tasks and channels
- Matrix operations for scientific computing
- Fast, declarative APIs
- Easy syntax and modular standard library

## Installation

**Prerequisites**
- Rust toolchain (1.70+)
- LLVM 14.0+
- CMake 3.20+

**Build**
```bash
git clone https://github.com/tanner-tobey/turtledog.git
cd turtledog
cargo build --workspace
```

## Usage

**Compile a BlueBird source file**
```bash
cargo run --bin bluebirdc -- examples/hello_world.bb
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT (see [LICENSE](LICENSE))