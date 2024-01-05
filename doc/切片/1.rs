fn main() {
    // 切片slice
    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];

    println!(">> {}, {}", hello, world);

    let word_index = first_world(&s);
    println!("{}", word_index);
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
