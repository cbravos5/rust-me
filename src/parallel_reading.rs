use std::collections::HashMap;

const STAR_SPANGLED_BANNER: [&str; 8] = [
  "O say can you see by the dawn's early light,",
  "What so proudly we hailed at the twilight's last gleaming,",
  "Whose broad stripes and bright stars through the perilous fight,",
  "O'er the ramparts we watched, were so gallantly streaming?",
  "And the rockets' red glare, the bombs bursting in air,",
  "Gave proof through the night that our flag was still there;",
  "O say does that star-spangled banner yet wave,",
  "O'er the land of the free and the home of the brave?",
];

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  // let lines_per_worker = input.len().div_euclid(worker_count);
  // let remainder = input.len().rem_euclid(worker_count);

  // let worker_line_count: Vec<usize> = vec![lines_per_worker, worker_count];

  // let fixed_workers_line_count = worker_line_count
  //   .iter()
  //   .enumerate()
  //   .map(|(i, x)| if i > remainder { *x + 1 } else { *x })
  //   .collect();

  let mut frequency_hashmap: HashMap<char, usize> = HashMap::new();

  for phrase in &input[0..7] {
    for ch in phrase.chars() {
      frequency_hashmap
        .entry(ch)
        .and_modify(|counter| {
          *counter += 1;
        })
        .or_insert(1);
    }
  }

  return frequency_hashmap;
}

pub fn execute() {
  let count = frequency(&STAR_SPANGLED_BANNER, 2);

  for pair in count.iter() {
    println!("{} = {}", pair.0, pair.1);
  }

  todo!("Count only letters and add shared memory parallelism with mutex")
}
