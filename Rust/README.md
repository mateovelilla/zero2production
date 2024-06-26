

### Recommendations
To build in windows create in folder .cargo/config.toml
```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```
To build in linux
```
# On Linux:
# - Ubuntu: sudo apt-get install lld clang
# - arch: sudo pacman -S lld clang
[target.x86_64-pc-unknown-linux-gnu]
rustflags = ["-C", "linker=clang","-C","link-arg=-fuse-ld=lld"]
```
On MacOs
```
# brew install michaeleisel/zld/zld 
```

#### Note: Try to use the unit tests in the same file and the integration test in external folder

### Tools
- [Rust Analyzer](https://rust-analyzer.github.io/)
- [Cargo Watch](https://crates.io/crates/cargo-watch)
- [Cargo Tarpaulin](https://github.com/xd009642/tarpaulin)
- [Cargo Clippy](https://github.com/rust-lang/rust-clippy)
- [Cargo Fmt](https://github.com/rust-lang/rustfmt)
- [Activex Web](https://github.com/actix/actix-web)
- [Bastion](https://www.bastion-rs.com/)
- [Cargo expand](https://github.com/dtolnay/cargo-expand)
- [Serde](https://serde.rs/)

## Useful Commands
- Expand code:
```
    cargo expand
```
- Install unstable compiler
```
rustup toolchain install nightly --allow-downgrade
```
- Change to default compiler
```
    rustup default
```

### Articles To Read

- [Understading Serde](https://www.joshmcguigan.com/blog/understanding-serde/)

### Concepts
- [Trunk-Based Development](https://www.atlassian.com/continuous-delivery/continuous-integration/trunk-based-development): ` is a version control management practice where developers merge small, frequent updates to a core “trunk” or main branch. Since it streamlines merging and integration phases, it helps achieve CI/CD and increases software delivery and organizational performance. In the early days of software development, programmers didn’t have the luxury of modern version control systems. Rather, they developed two versions of their software concurrently as a means of tracking changes and reversing them if necessary. Over time, this process proved to be labor-intensive, costly, and inefficient.`

- [Extractors(actix)](https://actix.rs/docs/extractors): facilities for type-safe request information access
- [Monomorphization](https://en.wikipedia.org/wiki/Monomorphization): is a compile-time process where polymorphic functions are replaced by many monomorphic functions for each unique instantiation.[1] It is considered beneficial to undergo the mentioned transformation because it results in the output intermediate representation (IR) having specific types, which allows for more effective optimization. Additionally, many IRs are intended to be low-level and do not accommodate polymorphism. The resulting code is generally faster than dynamic dispatch, but may require more compilation time and storage space due to duplicating the function body.
- CI Steps:
    - Tests
    ```
        cargo test
    ```
    - Code Coverage:
    ```
        cargo tarpaulin
    ```
    - Linting
    ```
        cargo clippy
    ```
    Achieve warnings in the pipelines with
    ```
        cargo clippy -- -D warnings
    ```
    -  Formatting
    ```
        cargo fmt -- --check
    ```
    - Security Vulnerabilities
    ```
        cargo audit
    ```
