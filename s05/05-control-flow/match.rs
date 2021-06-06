fn main() {
    let day = 5;
    match day {
        0 | 6 => println!("weekend"),
        1...5 => println!("weekday"), // 按这行输出
        _ => println!("invalid"),
    }
    /*
    其中|用于匹配多个值，...匹配一个范围 (包含最后一个值)，并且_在这里是必须的，
    因为match强制进行穷尽性检查 (exhaustiveness checking)，必须覆盖所有的可能值。
    如果需要得到|或者...匹配到的值，可以使用@绑定变量：
    */
    let x = 3;
    match x {
        e @ 1...5 => println!("got a range element {}", e), // 按这行输出，e是3
        _ => println!("anything"),
    }

    // 使用ref关键字来得到一个引用：
    let x = 5;
    let mut y = 5;
    match x {
        // the `r` inside the match has the type `&i32`
        ref r => println!("Got a reference to {}", r), // 按这行输出，r是5
    }
    match y {
        // the `mr` inside the match has the type `&i32` and is mutable
        ref mut mr => println!("Got a mutable reference to {}", mr), // 按这行输出，mr是5
    }

    // 再看一个使用match表达式来解构元组的例子：
    let pair = (0, -2);
    match pair {
        (0, y) => println!("x is `0` and `y` is `{:?}`", y), // 按这行输出，x是0，y是-2
        (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    // match的这种解构同样适用于结构体或者枚举。如果有必要，还可以使用..来忽略域或者数据：
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x), // 输出的x是0。忽略y的匹配，.. 相当于*
    }
    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);
    match x {
        // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"), // 按这行输出，x是5
        OptionalInt::Missing => println!("No such luck."),
    }

    // 此外，Rust还引入了if let和while let进行模式匹配：
    let number = Some(7);
    let mut optional = Some(0);
    // If `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number { // 注意这里是 =
        println!("Matched {:?}!", i); // 按这行输出，i是7。 这也就说明match类似于正则表达式了，像这里的i根本就没有显示声明
    } else {
        println!("Didn't match a number!");
    }
    // While `let` destructures `optional` into `Some(i)`, evaluate the block.
    while let Some(i) = optional {
        /*
        `i` is `0`. Try again.
        `i` is `1`. Try again.
        `i` is `2`. Try again.
        `i` is `3`. Try again.
        `i` is `4`. Try again.
        `i` is `5`. Try again.
        `i` is `6`. Try again.
        `i` is `7`. Try again.
        `i` is `8`. Try again.
        `i` is `9`. Try again.
        Greater than 9, quit!
        */
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i+1); // optional 会从Some(1)变更为Some(9)
        }
    }
}
