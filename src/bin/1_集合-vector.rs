/// Vector
/// cargo run --bin 1_集合-vector

fn main() {
    // 1 创建vec
    // 指定类型，无法推动
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // 2 创建vec
    // 写时，自动推断
    let mut v = Vec::new();
    v.push(100);
    v.push(200);
    v.push(300);

    // ------------------------------------
    // 读取vec
    let val = &v[1];
    println!("{}", val);

    match v.get(2) {
        Some(x) => println!("{}", x),
        None => println!("none ---"),
    }
}
