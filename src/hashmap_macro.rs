#[macro_export]
macro_rules! hashmap {
  ($($($x:expr => $y:expr)+ $(,)?)*) => {
    {
      let mut temp_hashmap = ::std::collections::HashMap::new();
      $($( temp_hashmap.insert($x,$y))*;)*
      temp_hashmap
    }
  };
}

pub fn execute() {
  let test_map = hashmap!(1 => 1, 2 => 5,);
  test_map.iter().for_each(|(k, v)| {
    println!("{} => {}", k, v);
  })
}
