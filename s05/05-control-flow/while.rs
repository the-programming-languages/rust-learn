fn main() {
    /*
    Rust中的while循环与C语言中的类似。
    对于无限循环，Rust有一个专用的关键字loop。
    如果需要提前退出循环，可以使用关键字break或者continue， 还允许在循环的开头设定标签 (同样适用于for循环)：
    */
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
     }

     println!("Exited the outer loop");
}