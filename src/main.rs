use anyhow::{Result, anyhow};
use std::collections::HashMap;

fn day1() -> Result<()> {
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


fn day2() -> Result<()> {
  let contents = std::fs::read("./src/inputtest2.txt")?;

  let string = std::str::from_utf8(&contents)?;

  let reports = string.lines();

  let mut safes = 0;

  'outer: for report in reports {
    let numbers_result: Result<Vec<_>, _> = report
      .split(" ")
      .map(|str| str.parse::<i32>())
      .collect();

    let numbers = numbers_result?;
    let mut prev = numbers[0];
    let mut prev_prev: Option<i32> = None;
    let mut increasing_opt: Option<bool> = None;
    let mut bad_levels_remaining: i32 = 1;

    let mut is_bad_level = |prev, curr: &i32| {
      if let None = increasing_opt {
        increasing_opt = Some(*curr > prev);
      }

      if let Some(increasing) = increasing_opt {
        if !increasing && *curr > prev {
          return true;
        } else if increasing && *curr < prev {
          return true;
        }

        let diff = if increasing {
          *curr - prev
        } else {
          prev - *curr
        };

        if diff > 3 {
          return true;
        } else if diff == 0 {
          return true;
        }
      }

      return false;
    };

    let mut is_bad_report = |_prev_prev: Option<i32>, prev: i32, curr: &i32| {
      if is_bad_level(prev, curr) {
        bad_levels_remaining -= 1;

        // if let Some(prev_prev) = prev_prev {
        //   if is_bad_level(prev_prev, curr) {
        //     return true
        //   }
        // }
      }

      if bad_levels_remaining == -1 {
        return true
      }

      return false
    };

    for number in &numbers[1..] {
      if is_bad_report(prev_prev, prev, number) {
        println!("it is bad report");
        continue 'outer;
      }

      if (bad_levels_remaining == 1) {
        prev = *number;
      }

      prev_prev = Some(prev);
    }

    safes += 1;
  }

  println!("{:#?}", safes);

  Ok(())
}

fn test() {
  let nums = vec![1,2,3,4,5];
  let nums2 = vec![1,2,3,4,5,6,7];

  'outer: for num in nums {
    for num2 in &nums2 {
      if *num2 == 3 {
        println!("it is 3, breaking outer.");

        break 'outer;
      }
      println!("{:#?}", *num2);
    }
  }
}

fn main() -> Result<()> {
  day2();
  Ok(())
}

/*
1 2
*/
