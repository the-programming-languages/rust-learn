/*
Rust中的for循环与C语言的风格非常不同，抽象结构如下：

for var in expression {
    code
}
其中expression是一个迭代器 (iterator)，具体的例子为0..10 (不包含最后一个值)， 或者[0, 1, 2].iter()。
*/
fn main() {
    println!("var: ");
    for var in 0..10 {
        println!("{}", var);
    }

    println!("array: ");
    for v in [0,1,2,3,4].iter() {
        println!("{}", v);
    }
}