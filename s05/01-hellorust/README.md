创建项目hellorust：
```shell
$ cargo new hellorust --bin
     Created binary (application) `hellorust` package
```

查看目录结构：
```shell
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```

修改main.rs，将内容改为：
```rust
fn main() {
    let rust = "Rust";
    println!("Hello, {}!", rust);
}
```

编译和运行：
```shell
# debug版本
$ cargo build # 仅编译，默认debug版本
   Compiling hellorust v0.1.0 (/Users/wslu/work/github/rust-learn/s05/hellorust/hellorust)
    Finished dev [unoptimized + debuginfo] target(s) in 2.37s

$ ./target/debug/hellorust # 运行
$ cargo run # 编译和运行合在一起

# release版本：
$ cargo build --release # 这个属于优化编译
   Compiling hellorust v0.1.0 (/Users/wslu/work/github/rust-learn/s05/hellorust/hellorust)
    Finished release [optimized] target(s) in 1.31s

$ ./target/release/hellorust # 如果前面是优化编译，则这样运行
$ cargo run --release # 同上，区别是是优化编译的
```



