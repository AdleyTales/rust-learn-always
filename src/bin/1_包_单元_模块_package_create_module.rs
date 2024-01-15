/// 方法 method
/// cargo run --bin 1_包_单元_模块_package_create_module

/*
    Package 和 Crate 和 Module

    在 Rust 中，crate 是代码打包、分发、编译以及复用的基本单元。

    独立编译单元：Rust 使用 crate 作为构建块来组织和编译项目。每个 crate 在编译时都会生成一个单独的目标文件，
    比如动态链接库（.so 或 .dylib 文件）或者静态库（.a 或 .rlib 文件），或者直接生成一个可执行文件。


    ## 一个Package: 【项目】
    - 包含1个Cargo.toml，他描述了如何构建这些Crates
    - 只能包含0-1个libary crate （最多一个libary）
    - 可以包含任意数量的binary crate
    - 但必须至少包含一个crate（libary或binary）


    ## Cargo的惯例
    - src/main.rs:
    - binary crate的crate root
    - crate名与package名相同

    - src/lib.rs
    - package包含一个libary crate
    - libary crate的crate root
    - crate名与package名相同

    - 一个Package可以同时包含src/main.rs和src/lib.rs
    - 一个binary crate 和 libary crate
    - 名称与package名相同

    - 一个Package可以有多个binary crate
    - 文件放在src/bin
    - 每个文件是单独的binary crate


    ## 定义Module来控制作用域和私有性
    - Module
    - 在一个crate内，将代码进行分组
    - 增加可读性，易于复用性
    - 控制项目（item）的私有性，public、private
    - 建立Module：
    - mod关键字
    - 可嵌套
    - 可包含其他项（struct、enum、常量、trait、函数等）的定义


    ## 路径 Path
    - 绝对路径 从crate root开始，使用crate名或字面值crate
    - 相对路径  从当前模块开始，使用self，super或当前模块的标识符


    ## 模块不仅可以组织代码，还可以定义私有边界
    - 默认是私有的

    ## super 关键字  父级别路径

    ## use 将路径导入到当前作用域

    ## 使用外部包（package）
    1、Cargo.tmol 添加依赖包package https://crates.io
    2、use将特定条目引入作用域
    [dependencies]
    rand = "0.8.5"
*/

fn main() {
    
}