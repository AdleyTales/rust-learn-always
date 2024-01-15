/// 方法 method
/// cargo run --bin 1_字符串_string


fn main() {
    //  String
    let mut s = String::from("Hello world"); // String

    let s = "hello rs";
    let s = s.to_string(); // String

    // 更新字符串
    // push_str() 字符串切片追加到String
    // push() 把单个字符附加到String
    // + 连接字符串

    // 拼接字符串 format!()
    let s1 = String::from("tic");
    let s2 = String::from("ad");

    let s = format!("{}-{}", s1, s2);
    println!("{}", s);
}