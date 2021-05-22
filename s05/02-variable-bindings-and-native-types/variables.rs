fn variables_binding() {
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2); // let 绑定 整数变量默认类型推断是 i32

    let b1:u32 = 5;
    println!("b1: {}", b1);
    // assert_eq!(a1, b1);
    //如果去掉上面的注释，在编译的时候会报错，因为类型不匹配
    /*
error[E0308]: mismatched types
 --> variables.rs:7:5
  |
7 |     assert_eq!(a1, b1);
  |     ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
    */
}

fn variables_mut() {
    let mut a:f64 = 1.0;
    let b = 2.0f32;

    // 改变a的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}

fn variables_let() {
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}

fn variables_types() {
    // boolean type
    let t = true;
    let f:bool = false;

    // char type
    let c = 'c';

    // numeric types
    let x = 42;
    let y:u32 = 123_456; // 实际的值为 123456, 数值类型可以使用_分隔符来增加可读性
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b1111_0000; // 二进制
    let oct = 0o7320_1546;
    let hex:i64 = 0xf23a_b049;

    // string types
    // 使用&符号将String类型转换成&str类型很廉价， 但是使用to_string()方法将&str转换到String类型涉及到分配内存， 除非很有必要否则不要这么做。
    let str = "Hello, world";
    let mut string = str.to_string();

    let str2 = r#"\\abcd##def//"#; //双引号中间的字符会保持原样，不会考虑转义
    println!("str2:{}", str2);

    // array and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros : [i64; 10] = [0; 10];

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    // raw pointers
    let x = 5;
    let raw = &x as *const i32; // Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
    let points_at = unsafe { *raw };

    // functions
    fn foo(x: i32) -> i32 { x }
    let bar: fn(i32) -> i32 = foo;

    // explicit conversion
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    // type aliases
    type NanoSecond = u64;
    type Point = (u8, u8);
}

fn main() {
    // 变量绑定
    variables_binding();

    // 可变绑定
    variables_mut();

    /*
例如： 固定大小类型：
1u8 1i8
1u16 1i16
1u32 1i32
1u64 1i64

可变大小类型：
1usize 1isize

浮点类型：
1f32 1f64
    */

    variables_let();
    variables_types();
}