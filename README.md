# learning_rust_from_go

## 前言

第一次使用 Rust 是在改寫區塊鏈 Side Project。主要是利用 [Hyperledger Sawtooth](https://www.hyperledger.org/use/sawtooth)，原先使用 Go，後來改用 Rust。發現 Rust 的速度有比較快，但不好學。這二、三年來斷斷續續重頭學習，也一直沒有好好整理。最近又再重頭學習一次，並且把公司內原本 CGO 的函式庫，用 Rust 重做一次後，比較有點心得。因此就利用我原本 Go 的筆記章節，一一記錄 Rust 在相關領域上，如何實現。

## 開發環境

- Rust 版本: 1.71.0
- 開發環境: Mac OS (arm64)
- 開發工具: [VSCode](https://code.visualstudio.com/)
- 文件使用 [Markdown Preview Enhanced](https://github.com/shd101wyy/markdown-preview-enhanced) 撰寫，請安裝完環境後再閱讀。
- [Source on Github](https://github.com/kigichang/learning_rust_from_go)

## 主要學習資源

- [Rust 程式設計, 2/e (Programming Rust: Fast, Safe Systems Development, 2/e)](https://www.tenlong.com.tw/products/9786263242326)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

### crates

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
- [clap](https://github.com/clap-rs/clap)
- [serde-rs](https://github.com/serde-rs/)
- [tokio.rs](https://tokio.rs/tokio/tutorial)
- [actix.rs](https://actix.rs/)
- [redis-rs](https://github.com/redis-rs/redis-rs)
- [tonic](https://github.com/hyperium/tonic)
- [gRPC-rs](https://github.com/tikv/grpc-rs)

### 其他資源

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/introduction.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
- [The bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/introduction.html)
- [Rust语言圣经(Rust Course)](https://course.rs/about-book.html)
- [PyO3 user guide](https://pyo3.rs/)
- [The \`wasm-bindgen\` Guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [The Rustonomicon](https://doc.rust-lang.org/beta/nomicon/leaking.html)
- [CXX — safe interop between Rust and C++](https://cxx.rs/index.html)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Rust语言中文社区](https://rustcc.cn/)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
- [How to bind (Python + NumPy) with (Rust + Ndarray)](https://itnext.io/how-to-bind-python-numpy-with-rust-ndarray-2efa5717ed21)

## Summary

- [本文件](README.md)

### 一、Rust 基礎說明

- Introduction
  - 參考文件
  - IDE 設定
  - 與 Go / C / Scala 簡單比較
- Syntax
  - 巨集 (macro __!__)
- Basic Type
  - Number
  - Boolean
  - String
  - Default Trait
- Function
- Aggregate Types
  - Vector
  - Struct
  - method (__impl__)
- Trait
  - impl trait for struct
  - __derive__
  - Debug, Display
- Advanced Type
  - enum and match
- Memory
  - Known Size and Free Once
  - Move / Copy / Clone / Drop
  - Borrow
  - Box
  - Rc and Weak
  - Cell / RefCell
  - OnceCell / OnceLock
- Error Handling
  - Option
  - Result
- Closure
  - Fn
  - FnOnce
  - FnMute
- Package
  - lib, bin and feature
  - pub and pub (crate)
  - testing
  - workspace
- cargo and build script
  - cfg and target
  - static and dynamic link

### 二、多執行緒

- Arc and Mutex
- tokio
  - async
  - thread
  - channel
    - select!
  - actor

### 三、實作應用

- Serde json and toml
- clap
- log4rs
- actix
- Diesel
- http client
- websocket client
- time package
  - std::time
  - chrono

### 四、進階應用

- [FFI and cbindgen](https://rust-lang.github.io/rust-bindgen/introduction.html)
- [PyO3](https://github.com/PyO3/pyo3)
