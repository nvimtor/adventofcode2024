use anyhow::{Result, anyhow};
use std::collections::HashMap;

fn main() -> Result<()> {
  let contents = std::fs::read("./src/input.txt")?;

  let string = std::str::from_utf8(&contents)?;

  let lines = string.lines();

  let mut lefts: Vec<&str> = vec![];
  let mut rights: Vec<&str> = vec![];
  let mut rights_count: HashMap<&str, i32> = HashMap::new();

  for line in lines {
    let split: Vec<&str> = line.split("   ").collect();

    let left = *split.get(0).ok_or(anyhow!("expected left"))?;

    lefts.push(left);

    let right = *split.get(1).ok_or(anyhow!("expected right"))?;

    rights.push(right);
    *rights_count.entry(right).or_insert(0) += 1;
  }

  lefts.sort();
  rights.sort();

  let lefts_iter = lefts.iter();
  let rights_iter = rights.iter();
  let zip_iter = lefts_iter.zip(rights_iter);

  let mut total_diff: i32 = 0;
  let mut total_similarity_score = 0;

  for (left, right) in zip_iter {
    let left_num: i32 = left.parse()?;
    let right_num: i32 = right.parse()?;

    let mut diff = 0;

    if left_num > right_num {
      diff = left_num - right_num;
    } else if left_num < right_num {
      diff = right_num - left_num;
    }

    total_diff += diff;
  };

  for left in lefts {
    let mut count = 0;

    if let Some(right_count) = rights_count.get(left) {
      count = *right_count;
    }

    let num: i32 = left.parse()?;


    total_similarity_score += num * count;
  };

  println!("{:#?}", total_diff);
  println!("{:#?}", total_similarity_score);

  Ok(())
}
