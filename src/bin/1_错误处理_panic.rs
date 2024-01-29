/// 错误处理 panic
/// cargo run --bin 1_错误处理_panic
/*

 # Rust可靠性：错误处理 | 大部分情况下，在编译时提示错误，并处理


 ## 错误的分类
 - 可恢复 - 例如文件未找到，可再次尝试
 - 不可恢复 - 例如访问索引超出范围


 ## Rust没有类似异常的机制
 - 可恢复错误：Rust<T,E>
 - 不可恢复： panic!宏


 ## Result枚举
 enum Result<T,E> {
    Ok(T),
    Err(E),
 }

 match x {}

 unwrap 如果成功，直接返回Ok 相当于match表达式的快捷方式 | 缺点：错误信息不可自定义
 expect 和unwrap非常类似，可以指代panic!自定义的错误信息


 ## 传播错误 将错误传给调用者
 ? 运算符 传播错误的快捷方式 | 可以消除大量的代码
 fn read_file() -> Result<String, io::Error> {
    ...
 }

 ## 总体原则
 - 在定义一个可能失败的函数时，优先考虑返回Result
 - 否则就panic!

 Example: 
 1、编写示例、原型代码、测试时：可以使用panic! 
    - 演示某些概念： unwrap
    - 原型代码： unwrap、expect
    - 测试：unwrap、expect

 2、

*/
use std::fs::File;
use std::io;
use std::io::Read;

fn main1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error open file: {:?}", error)
        }
    };

    println!("{:?}", f);

    // unwrap 等价
    let f = File::open("hello.txt").unwrap();
    println!("{:?}", f);

    // expect
    let f = File::open("hello.txt").expect("无法打开文件-10009");
    println!("{:?}", f);
}

fn main() {
    let f = read_file().unwrap();
    println!("{:?}", f);

    println!("--------------------------------");

    let f = read_file2().unwrap();
    println!("{:?}", f);
}

fn read_file() -> Result<String, io::Error> {
    let f = File::open("D:\\hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

///
/// 使用?传播 快捷方式 等价上面 [Very Good!]
/// 
fn read_file2() -> Result<String, io::Error> {
    let mut f = File::open("D:\\hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
