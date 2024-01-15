/// 模式匹配 option
/// cargo run --bin 1_模式匹配_option

/// 内置 Option

/*

## 标准库中的定义：
- enum Option<T> {
    Some(T),
    None,
}

## Option比NULL好

## match匹配必须穷举所有的可能
match v {
    1 => xxx,
    _ => ()
}

## if let 可以看做是match的语法糖（只关心一种匹配而忽略其他匹配的情况）
也可以搭配else使用

*/

fn main() {
    let v= Some(3);
    match v {
        Some(3) => println!("Three ..."),
        // _ => (),
        _ => println!("Others ..."),
    }

    // 等价 其他的不关心
    if let Some(3) = v {
        println!("Three ...")
    }else {
        println!("Others ...")
    }
}