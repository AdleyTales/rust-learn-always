/// 发布配置 publish
/// cargo run --bin 2_发布配置_publish

/*
  release profile:
  - 预定义的
  - 可自定义：可使用不同的配置，对代码编译拥有更多的控制

  # Cargo主要的两个profile：
  - dev profile：适用于开发，cargo build
  - release profile: 适用于发布， cargo build --release

  [profile.dev]
  opt-level: 0;

  [profile.release]
  opt-level: 3;

  opt-level: rust编译器优化级别

  # crates.io

  # 文档注释：用于生产文档
  - 生成HTML文档
  - 使用///
  - 支持markdown

  cargo doc --open

  # //! 描述外层条目 描述包、模块
  

*/

fn main() {
  
}
