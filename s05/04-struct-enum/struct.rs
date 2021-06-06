fn struct_part1() {
    /*
    结构体 (struct) 是一种记录类型，所包含的每个域 (field) 都有一个名称。 每个结构体也都有一个名称，
    通常以大写字母开头，使用驼峰命名法。 元组结构体 (tuple struct) 是由元组和结构体混合构成，元组结构体有名称，但是它的域没有。
    当元组结构体只有一个域时，称为新类型 (newtype)。 没有任何域的结构体，称为类单元结构体 (unit-like struct)。
    结构体中的值默认是不可变的，需要给结构体加上mut使其可变。
    */

    // structs
    struct Point {
        x: i32,
        y: i32,
    };
    let point: Point = Point { x: 0, y: 0 };
    let point = Point { x: 0, y: 0 };

    // tuple struct
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, yellow, blue) = android_green;

    // A tuple struct’s constructors can be used as functions.
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    // newtype: a tuple struct with only one element
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("integer_length = {}", integer_length);

    // unit-like structs
    struct EmptyStruct;
    let empty = EmptyStruct; // 等同于 EmptyStruct{}
}

fn struct_part2() {
    // 一个包含..的struct可以用来从其它结构体拷贝一些值或者在解构时忽略一些域：
    #[derive(Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    };

    let origin = Point3d::default();
    println!("origin:{}", origin);
    let point = Point3d { y: 1, ..origin };
    println!("point:{}", point);
    let Point3d { x: x0, y: y0, .. } = point;
    // println!("point:{}", point);
    // println!("origin:{} point:{}", origin, point);
}

fn main() {
    struct_part1();
    struct_part2();
}
