/// 方法 method
/// cargo run --bin 1_方法_method

/*
从面向对象语言过来的同学对于方法肯定不陌生，class 里面就充斥着方法的概念。
在 Rust 中，方法的概念也大差不差，往往和对象成对出现：
object.method()
例如读取一个文件写入缓冲区，如果用函数的写法 read(f, buffer)，用方法的写法 f.read(buffer)。
不过与其它语言 class 跟方法的联动使用不同（这里可能要修改下），Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用，特征将在后面几章进行介绍。
*/

fn main() {
    let c = Circle::new(12.0, 8.0, 5.0);
    println!("x: {}, y: {}, area: {}", c.get_x(), c.get_y(), c.area());
}

// 定义方法
// Rust 使用 impl 来定义方法
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }

    // getter x
    fn get_x(&self) -> f64 {
        self.x
    }

    // getter y
    fn get_y(&self) -> f64 {
        self.y
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        let r = std::f64::consts::PI * self.radius * self.radius;
				println!("{}, {}", std::f64::consts::PI, r);
				r
    }
}
