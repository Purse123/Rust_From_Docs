# Chapter 1
when installing rust by:
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

**we install rust by rustup command line tool**
it install component:
- **cargo** -> package manager for rust just like npm in js
- **clippy** -> improve code quality by catching common mistake
- **rust-docs** -> refers to official rust docs
- **rust-std** -> collection of core library that comes with rust
    -> std::io    // input/output handling
    -> std::fs    // file system operations 
    -> std::thread  // for concurrency and multithreading
    -> std::collections   // for common data structures like Vec, HashMap, and BTreeMap
- **rustc** -> rust compiler, cargo use rust behind the scenes
- **rustfmt** -> official rust formatter

***
- rust style is to **indent with four space instead of tab**
***
```rust
println!("Hello, world!");
```
- here **println!** is rust **macros**.
- the macro replace the macro with code
