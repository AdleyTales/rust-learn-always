/// 切片 slice
/// cargo run --bin 1_切片_slice

/*
    Rust的另外一种不持有所有权的数据类型：切片(slice)

    ## 将字符串切片作为参数传递
    ```rs
    fn first_world(s: &String) -> $str { ... }
    ```
    - 有经验的Rust开发者会采用&str作为参数类型，因为这样就可以同时接收String和&str类型的参数了：
    ```rs
    fn first_world(s: &str) -> $str { ... }
    ```
    - 使用字符串切片，直接调用该函数
    - 使用String，可以创建一个完整的String切片来调用该函数

    - 定义函数时使用字符串切片来代替字符串引用会使我们的AAAPI更加通用，且不会损失任何功能
*/

fn main() {
    // 切片slice
    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];

    println!("{}, {}", hello, world);

    let word_index = first_world(&s);
    println!("first word len is {}", word_index);
}

// 获取第一个词的长度
fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
