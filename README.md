# zero2production
This is a repository to keep all important that I learn of the ZERO TO PRODUCTION IN RUST book
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

### Tools
- [Rust Analyzer](https://rust-analyzer.github.io/)
- [Cargo Watch](https://crates.io/crates/cargo-watch)
- [Cargo Tarpaulin](https://github.com/xd009642/tarpaulin)
- [Cargo Clippy](https://github.com/rust-lang/rust-clippy)
- [Cargo Fmt](https://github.com/rust-lang/rustfmt)

### Concepts
- [Trunk-Based Development](https://www.atlassian.com/continuous-delivery/continuous-integration/trunk-based-development): ` is a version control management practice where developers merge small, frequent updates to a core “trunk” or main branch. Since it streamlines merging and integration phases, it helps achieve CI/CD and increases software delivery and organizational performance. In the early days of software development, programmers didn’t have the luxury of modern version control systems. Rather, they developed two versions of their software concurrently as a means of tracking changes and reversing them if necessary. Over time, this process proved to be labor-intensive, costly, and inefficient.`
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
