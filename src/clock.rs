use core::fmt;

#[derive(Debug)]
struct Clock {
  minutes: i32,
}

impl Clock {
  fn add(&self, minutes: i32) -> Self {
    Self { minutes: self.minutes + minutes }
  }
  fn subtract(&self, minutes: i32) -> Self {
    Self { minutes: self.minutes - minutes }
  }
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let minutes = self.minutes.rem_euclid(60);
    let hours = self.minutes.div_euclid(60).rem_euclid(24);

    write!(f, "{:02}:{:02}", hours, minutes)
  }
}

pub fn execute() {
  let clock = Clock { minutes: 615 };
  let first_add = clock.add(30);
  let second_add = first_add.add(30);
  let third_add = second_add.add(780);
  let first_subtract = third_add.subtract(30);

  println!("{}", format!("It's {clock}"));
  println!("{}", format!("It's {first_add}"));
  println!("{}", format!("It's {second_add}"));
  println!("{}", format!("It's {third_add}"));
  println!("{}", format!("It's {first_subtract}"));
}
