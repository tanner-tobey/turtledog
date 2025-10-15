[workspace]

# This section defines the members of the workspace.
# Cargo will know to look for the compiler in src/bluebirdc and the runtime in runtime/.
members = [
    "src/bluebirdc",
    "runtime",
]

# You can also define shared profile settings here for all crates.
# For example, enabling optimizations for release builds.
[profile.release]
lto = true          # Enable Link-Time Optimization for better performance
codegen-units = 1   # Produce a single compilation unit for max optimization
panic = "abort"     # Abort on panic for smaller executables
