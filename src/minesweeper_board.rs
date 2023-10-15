fn check_mine(minefield: &[&[u8]], i: i32, j: i32) -> usize {
  match (i, j, minefield) {
    (i, j, _) if i < 0 || j < 0 => 0,
    (i, j, minefield) if i >= (minefield.len() as i32) || j >= (minefield[0].len() as i32) => 0,
    (i, j, minefield) if minefield[i as usize][j as usize] == b'*' => 1,
    (_, _, _) => 0,
  }
}

fn annotate(minefield: &[&str]) -> Vec<String> {
  if minefield.is_empty() {
    return vec![];
  }

  if minefield[0].is_empty() {
    return vec!["".to_string()];
  }

  let mut board: Vec<char> = vec![];

  let bytes_minefield: Vec<&[u8]> = minefield
    .iter()
    .map(|x| x.as_bytes())
    .collect();

  for (i, row) in bytes_minefield.iter().enumerate() {
    for (j, col) in row.iter().enumerate() {
      if *col != b' ' {
        board.push('*');
        continue;
      }

      let mut mine_count = 0;

      for y in -1..2 {
        for x in -1..2 {
          mine_count += check_mine(bytes_minefield.as_slice(), (i as i32) + y, (j as i32) + x);
        }
      }

      board.push(
        if mine_count > 0 {
          char::from_digit(mine_count as u32, 10).unwrap()
        } else {
          ' '
        }
      );
    }
  }

  board.chunks(minefield[0].len()).map(String::from_iter).collect()
}

pub fn execute() {
  let board = annotate(&[" *  * ", "  *   ", "    * ", "   * *", " *  * ", "      "]);

  println!("{:?}", board)
}
