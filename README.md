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
