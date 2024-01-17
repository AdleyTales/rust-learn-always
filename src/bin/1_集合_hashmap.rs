use std::collections::HashMap;

/// hashmap
/// cargo run --bin 1_集合_hashmap

// 数据存储在head上

fn main() {
    // 创建HashMap
    let mut map = HashMap::new();

    // 添加数据 insert()
    map.insert("name", "adley");
    map.insert("score", "100");

    println!("{:?}", map); // {"name": "adley"}

    // 遍历
    for (k, v) in &map {
        println!("{}: {}", k, v)
    }

    println!("---------------------------------------------------");

    // 更新
    map.entry("adley").or_insert("tales");
    map.entry("adleyT").or_insert("tales"); // 如果没有查到则插入

    // 遍历
    for (k, v) in &map {
        println!("{}: {}", k, v)
    }
}
