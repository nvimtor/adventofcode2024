#![feature(try_blocks)]
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
  println!("started");

  let contents = std::fs::read("./src/input2.txt")?;

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

    for number in &numbers[1..] {
      let mut is_bad_level = false;

      if let None = increasing_opt {
        increasing_opt = Some(*number > prev);
      }

      if let Some(increasing) = increasing_opt {
        if !increasing && *number > prev {
          is_bad_level = true;
        } else if increasing && *number < prev {
          is_bad_level = true;
        }

        let diff = if increasing {
          *number - prev
        } else {
          prev - *number
        };

        if diff > 3 {
          is_bad_level = true;
        } else if diff == 0 {
          is_bad_level = true;
        }
      }

      if is_bad_level {
        bad_levels_remaining -= 1;
      }

      // if bad_levels_remaining == 1 {
        prev = *number;
      // }

      if bad_levels_remaining == -1 {
        continue 'outer;
      }

      // prev_prev = Some(prev);
    }

    println!("{:#?} is OK", report);
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

/*
TODO create shitty parser for day 3
Using nom would be too easy
*/

type Parser<'a, A> = Box<dyn Fn(&'a str) -> (&'a str, anyhow::Result<A>) +'a>;

fn char<'a>(expected: char) -> Parser<'a, char> {
  Box::new(move |input: &'a str| {
    let mut out: &str = "";

    let result: Result<char> = try {
      let head = input.chars().next().ok_or(anyhow!("expected string to not be empty"))?;

      if head == expected {
        anyhow::Ok(head)?
      } else {
        Err(anyhow!("{:#?} does not match {:#?}", head, expected))?
      }
    };

    if let Ok(head) = result {
      out = &input[head.len_utf8()..];
    }

    (out, result)
  })
}

/*
we can make it recursive:

mumul

mu

as soon as it hits m, it wil generate a parser with

("mul", Err)

so now, we just need to call the same parser again, hoping it will get it right this time.

if not, recurse
*/

fn string<'a>(expected: &'a str) -> Parser<'a, &'a str> {
  Box::new(move |input: &'a str| {
    let parsers = expected.chars().map(char);

    let mut last_out: &str = input;
    let mut result: Result<&'a str> = Ok(expected);

    for parser in parsers {
      let (out, p_result) = parser(last_out);

      if let Err(err) = p_result {
        result = Err(err);
        break
      };

      last_out = out;
    }

    (last_out, result)
  })
}

fn map<'a, A: 'a, B, F>(fa: Parser<'a, A>, ab: F) -> Parser<'a, B>
where
F: Fn(A) -> B + Copy + 'a
{
  Box::new(move |input: &'a str| {
    let (out, result) = fa(input);
    let res2 = result.map(ab);

    (out, res2)
  })
}

fn mul<'a>() -> Parser<'a, &'a str> {
  fn go<'a>(input: &'a str) {

  }


  Box::new(move |input: &'a str| {
    let q = go(input);

    let (out, res) = string("mul")(input);

    // let x = q("a");

    todo!()
  });

  // q
}

fn day3() -> Result<()> {
  println!("Day 3!");

  let test_str = "mumul";
  let p = char('a');
  let strparser = string("mu");

  let anotherone = map(strparser, |res| {
    "opa"
  });

  let result = anotherone(test_str);

  println!("{:#?}", result);

  Ok(())
}

fn main() -> Result<()> {
  day3();
  Ok(())
}

/*
string("mul")
mumul
*/
