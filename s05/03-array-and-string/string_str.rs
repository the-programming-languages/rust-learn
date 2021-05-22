fn main() {
    // str
    // 字符串字面值
    let hello = "Hello World";
    let hello: &'static str = "Hello World";

    // String
    // 创建一个空的字符串
    let mut s = String::new();
    // 从 `&str` 类型转化成 `String` 类型
    let mut hello = String::from("Hello, ");
    // 压入字符和压入字符串切片
    hello.push('w');
    hello.push_str("orld");

    // 弹出字符
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}