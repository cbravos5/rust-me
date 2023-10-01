fn get_abs_sum(values_array: Vec<i32>) -> i32 {
   values_array.iter().map(|x| x.abs()).sum()
}

pub fn execute() {
    println!("{}", get_abs_sum(vec!(2, -1, 4, 8, 10)));
    println!("{}", get_abs_sum(vec!(-3, -4, -10, -2, -3)));
    println!("{}", get_abs_sum(vec!(2, 4, 6, 8, 10)));
    println!("{}", get_abs_sum(vec!(-1)));
}
