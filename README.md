#### This is multi project repo so you have to target specific project.

> build_check.rs

```rust
Build project
cargo build --bin=build_check
Run build_check binary
cargo run --bin=build_check
Check for errors
cargo check --bin=build_check
```

> codegen.rs

```rust
Build project
cargo build --bin=codegen
Run codegen binary
cargo run --bin=codegen
Check for errors
cargo check --bin=codegen
```

> analyzer.rs

```rust
Build project
cargo build --bin=analyzer
Run analyzer binary
cargo run --bin=analyzer
Check for errors
cargo check --bin=analyzer
```

[Multi binary / executable project](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target)

[Cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)

> This will essentially compile the packages without performing the final step of code generation, which is faster than running cargo build.
