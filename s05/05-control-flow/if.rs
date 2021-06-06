fn main() {
    /*
    If是分支 (branch) 的一种特殊形式，也可以使用else和else if。
    与C语言不同的是，逻辑条件不需要用小括号括起来，但是条件后面必须跟一个代码块。
    Rust中的if是一个表达式 (expression)，可以赋给一个变量：
    */
    let x = 5;
    let y = if x == 5 {10} else {15};
    println!("x: {}, y:{}", x,y);

    /*
    Rust是基于表达式的编程语言，有且仅有两种语句 (statement)：
    1. 声明语句 (declaration statement)，比如进行变量绑定的let语句。
    2. 表达式语句 (expression statement)，它通过在末尾加上分号;来将表达式变成语句，
       丢弃该表达式的值，一律返回unit()。

    表达式如果返回，总是返回一个值，但是语句不返回值或者返回()，所以以下代码会报错：
    19 |     let z: i32 = if x == 5 { 10; } else { 15; };
   |                                         ^^^^-^^
   |                                         |   |
   |                                         |   help: consider removing this semicolon
   |                                         expected `i32`, found `()`
    */
    let y = (let x = 5);
    let z: i32 = if x == 5 { 10; } else { 15; };
    // 值得注意的是，在Rust中赋值 (如x = 5) 也是一个表达式，返回unit的值()。


}