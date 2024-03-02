## Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source "$HOME/.cargo/env"
```

## LLVM Libraries
```
wget https://github.com/llvm/llvm-project/releases/download/llvmorg-15.0.6/clang+llvm-15.0.6-x86_64-linux-gnu-ubuntu-18.04.tar.xz
tar Jxf clang+llvm-15.0.6-x86_64-linux-gnu-ubuntu-18.04.tar.xz
mv clang+llvm-15.0.6-x86_64-linux-gnu-ubuntu-18.04 llvm15.0
```
```
echo 'export PATH=~/llvm15.0/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

## Build

```
git clone https://github.com/Sin7Y/ola-lang
cd ola-lang
cargo build --release
```

## Clear cargo build cache and re-build
```
cargo clean
cargo build --release
```

->> target/release/olac

### Alternative
```
cargo install olac
```


