/// 特征 trait
/// cargo run --bin 1_特征_trait

/*
  # Trait

  - Trait告诉Rust编译器：
    - 某种类型具有哪些并且可以与其他类型共享的功能
  - Trait：抽象的定义共享行为
  - Trait bounds(约束)：泛型类型参数指定为实现了特定行为的类型
  - Trait与其他语言的接口（interface）类似，但有些区别

  ## 定义一个Trait
  Trait定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为
  - 关键字：trait
  - 只有方法签名，没有具体实现
  - trait可以有多个方法：每个方法签名占一行，以；结尾

  ## 在类型上实现trait
  - 与为类型实现方法类似
  - 不同之处：
    - impl Xxxxx for Tweet {...}
    - 在impl的块里，需要对Trait里的方法签名进行具体的实现

  impl Trait语法： 适用于简单情况
  - impl Trait 语法是Trait bound的语法糖

  Trait Bound语法：可用于复杂情况

  使用+指定多个Trait bound （可以使用Where语法））
  pub fn notify(item: impl Summary + Display) {...}
  pub fn notify2<T: Summary + Display>(item: T) {...}
  pub fn notify3<T,U>(item: T) -> String
  where 
    T: Summary + Dispaly,
    U: Clone + Debug
  {...}
*/

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("headline: {}!", self.headline)
    }
}


// impl Trait 语法
// 调用方法 
// 实现了Summary这个trait的方法
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize())
}

// Trait Bound 语法
pub fn notify2<T: Summary>(item: T) {
  println!("Breaking2 news! {}", item.summarize())
}

fn main() {
  let news = NewsArticle{
    headline: String::from("标题"),
  };

  println!("{}", news.summarize());

  // notify(news);
  notify2(news);
}
