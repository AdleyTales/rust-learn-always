/// 测试 test
/// cargo run --bin 2_测试_test
/// cargo test --bin 2_测试_test

/*
  # 测试（函数） Rust一个测试就是一个函数

  测试：
  - 函数
  - 验证非测试代码的功能是否和预期一致

  ## 测试函数体（通常）执行的3个操作： 3A操作
  - 准备数据/状态
  - 运行被测试的代码
  - 断言（Assert）结果

  ## 运行测试 cargo test 命令会运行所有测试函数


*/

fn main() {
  
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_words() {
    assert_eq!(2+3, 5)
  }
}