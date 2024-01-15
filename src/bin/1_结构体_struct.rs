/// 结构体 struct
/// cargo run --bin 1_结构体_struct

/*
    自定义的数据类型, 为相关联的值命名，打包 => 有意义的组合


    ## struct更新语法
    - 当你想基于某个struct实例来创建一个新实例的时候，可以使用struct更新语法：
    ```rs
    let user2 = User {
        email: String::from("abc@126.com"),
        username: String::from("djsklajdkl"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    }

    => 更新语法 简化

    let user2 = User {
        email: String::from("abc@126.com"),
        username: String::from("djsklajdkl"),
        ..user1
    }
    ```


    ## Tuple struct
    - 可定义类似tuple的struct，叫做tuple struct
    - Tuple struct整体有个名，但里面的元素没有名
    - 适用：想给整个tuple起名，并让它不同于其他tuple，而且又不需要给每个元素起名
    - 例子：
    ```rs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    ```
    - black和origin是不同的类型，是不同tuple struct的实例。


    # struct 数据的所有权
    ```rs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    ```
    - 这里的字段适用了String而不是&str：
    - 该struct实例拥有其所有的数据
    - 只要struct实例是有效的，那么里面的字段数据也是有效的
    - struct里也可以存放引用，但这需要适用生命周期
    - 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的
    - 如果struct里面存储引用，而不使用生命周期，就会报错
*/

fn main() {
    // 2 实例化struct
    // 为每个字段指定具体值
    // 无需按声明的顺序进行指定
    // 一旦struct的实例是可变的，那么实例中所有的字段都是可变的
    let u = build_user("abc@126.com", "adleytales");
    println!("user {:?}", u);
}

// 1 定义struct
// 使用struct关键字，并为整个struct命名
// 在花括号内，为所有字段(Field)定义名称和类型
#[derive(Debug)]
struct User<'a> {
    username: &'a str,
    email: &'a str,
}

fn build_user<'a>(email: &'a str, username: &'a str) -> User<'a> {
	User {
		email,
        username,
	}
}