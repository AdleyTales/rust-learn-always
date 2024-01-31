/// 迭代器 iterator
/// cargo run --bin 2_迭代器_iterator

/*
  # Iterator trait
  - 所有的迭代器都实现了Iterator trait
  - Iterator trait仅要求实现一个方法: next
  - next: 
    - 每次返回迭代器中的一项
    - 返回结果包裹在Some里
    - 迭代结束：返回None
  - 可直接在迭代器上调用next方法

*/

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("got: {}", val);
    // }

   assert_eq!(v1_iter.next(), Some(&1));  
   assert_eq!(v1_iter.next(), Some(&2));  
   assert_eq!(v1_iter.next(), Some(&3));  

   let v2: Vec<u32> = v1.iter().map(|x| x+1).collect();
   println!("v2: {:?}", v2); // v2: [2, 3, 4]
}
