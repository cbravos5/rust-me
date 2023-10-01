use std::{ collections::HashMap, thread };

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

fn count_chars(lines: &[&str]) -> HashMap<char, usize> {
  let mut frequency_map: HashMap<char, usize> = HashMap::new();

  for line in lines {
    for ch in line.to_lowercase().chars() {
      if ch.is_alphabetic() {
        frequency_map
          .entry(ch)
          .and_modify(|count| {
            *count += 1;
          })
          .or_insert(1);
      }
    }
  }

  frequency_map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  match input.len() {
    0 => HashMap::new(),
    _ =>
      thread::scope(|scope| {
        let input_chunks = input.chunks(input.len() / worker_count + 1);

        let mut handles = vec![];

        for chunk in input_chunks {
          let handle = scope.spawn(|| count_chars(chunk));
          handles.push(handle);
        }

        let mut frequency_map = handles.pop().unwrap().join().unwrap();

        for result in handles {
          result
            .join()
            .unwrap()
            .into_iter()
            .for_each(|(ch, value)| {
              frequency_map
                .entry(ch)
                .and_modify(|count| {
                  *count += value;
                })
                .or_insert(value);
            });
        }

        frequency_map
      }),
  }
}

pub fn execute() {
  let count = frequency(&STAR_SPANGLED_BANNER, 2);

  for pair in count.iter() {
    println!("{} = {}", pair.0, pair.1);
  }
}
