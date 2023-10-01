fn double_swap(original_string: String, char_a: char, char_b: char) -> String {
  original_string.chars().map(|c|
    match c {
      _ if c == char_a => char_b,
      _ if c == char_b => char_a,
      _ => c
    }
  ).collect::<String>()
}

pub fn execute() {
  println!("{}", double_swap(String::from("aaabbb"), 'a', 'b'))
}
