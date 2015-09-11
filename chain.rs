// This program is written as a solution to an r/dailyprogrammer challenge:
// https://www.reddit.com/r/dailyprogrammer/comments/2y5ziw/20150306_challenge_204_hard_addition_chains/

// Notes: 
//  - be sure to compile with optimizations on (rustc -o), as it makes a huge difference, up to 100x faster from my brief testing
//  - on my machine, this solves the second bonus input (25, 1234567) in about ten seconds

use std::io;
use std::io::Write;
use std::ops::Index;

#[derive(Debug)]
struct Chain {
  values: Vec<i32>,
  sum: i32,
}

use std::num;

impl Chain {
  fn new() -> Chain {
    Chain{
      values: vec![1],
      sum: 1,
    }
  }
  
  fn add(&mut self, to_add: i32) {
    self.values.push(to_add);
    self.sum += to_add;
  }
  
  fn length(&self) -> i32 {
    (self.values.len() as i32)
  }     
  
  // note: chain length is number of additions, which is one less than the length of the vec
  fn min_sum(&self, target_length: i32) -> i32 {
    let delta: i32 = target_length - self.length();
    let biggest: i32 = *(self.values.last().unwrap());
    let next_n: i32 = (biggest - 1) * (delta + 1) + sum_n_2(delta + 1) - biggest; // subtract biggest so you don't double-count it
    
    next_n + self.sum 
  }
  
  fn max_sum(&self, target_length: i32) -> i32 {
    let delta: i32 = target_length - self.length();
    let biggest: i32 = *(self.values.last().unwrap()); 
    
    return biggest * (pow(2, delta + 1) - 2) + self.sum    
    // sum of 2^n = 2^(n+1) - 1
    // multiply by biggest
    // subtract biggest so you don't double count it, and then add the current sum
  }
  
  fn possible_next_values(&self) -> Vec<i32> {
    // making the assumption that the next value is larger than the prevous largest one
    // since the same chain could have been generated in a different order to make this true
    let mut result = vec![];
    let biggest: i32 = *(self.values.last().unwrap());
    
    // rewrite with iterators: faster?
    for i in 0..self.length() {
      for j in i..self.length() {
        let next: i32 = self.values[i as usize] + self.values[j as usize];
        if next <= biggest {
          continue; // chain must be increasing
        }
        // filter repeats of numbers that are already in the vec
        if (result.len() == 0) || (next > *result.last().unwrap()) {   
          result.push(next)
        }
      }
    }
    
    result
  }
}

impl Clone for Chain {
  fn clone(&self) -> Chain {
    Chain {values: self.values.clone(), sum: self.sum}
  }
}

impl Index<usize> for Chain {
  type Output = i32;
  
  fn index(&self, _index: usize) -> &i32 { // may need _index to be u32, may need to return &i32
    &self.values[_index]
  }
}

fn solve(target_length: i32, target_sum: i32) -> Option<Chain> {
  let c: Chain = Chain::new();
  
  try_chain(target_length, target_sum, c)
}

fn try_chain(target_length: i32, target_sum: i32, c: Chain) -> Option<Chain> {
  if c.length() == target_length {
    if c.sum == target_sum {
      Some(c)  
    }
    else {
      None
    }
  }
  else {  
    for i in c.possible_next_values() {
      let mut c_copy  = c.clone();
      c_copy.add(i);
      
      if target_sum < c_copy.min_sum(target_length) {
        continue; // Forced to over-shoot target sum in the given length
      }
      if target_sum > c_copy.max_sum(target_length) {
        continue; // Can't reach the target sum in the given length
      }
      
      let x: Option<Chain> = try_chain(target_length, target_sum, c_copy);
      
      if x.is_some() {
        return x
      }
    }
    None
  }
}

fn main() {
  // read chain length
  let mut input: String = String::new();
  print!("Enter chain length:  ");
  io::stdout().flush().ok().expect("Could not flush stdout");
  io::stdin().read_line(&mut input).ok().expect("Failed to read line");
  let target_length: i32 = input.trim().parse().ok().expect("Could not parse a number");
  
  //read target sum
  let mut input2: String = String::new();
  print!("Enter target sum:  ");
  io::stdout().flush().ok().expect("Could not flush stdout");
  io::stdin().read_line(&mut input2).ok().expect("Failed to read line");
  let target_sum: i32 = input2.trim().parse().ok().expect("Could not parse a number");
  
  let solution = solve(target_length, target_sum);
  
  match solution {
    Some(s) => println!("{:?}", s.values),
    None    => println!("No solution."),
  }
}

fn pow(base: i32, exp: i32) -> i32 {
  let mut result: i32 = 1;
  for i in 0..exp {
    result *= base;
  }
  
  result
}

fn sum_n_2(n: i32) -> i32 {
  n*(n+1)/2
}

#[test]
fn test_solve() {
  let mut solution = solve(2, 3);
  assert!(solution.is_some());
  solution = solve(5, 18);
  assert!(solution.is_some());
  solution = solve(5, 19);
  assert!(solution.is_some());
  solution = solve(5, 20);
  assert!(solution.is_some());
  solution = solve(5, 31);
  assert!(solution.is_some());
  solution = solve(5, 15);
  assert!(solution.is_some());
  
  solution = solve(5, 32);
  assert!(!solution.is_some());
  solution = solve(5, 14);
  assert!(!solution.is_some());
  solution = solve(5, 30);
  assert!(!solution.is_some());
  
  solution = solve(10, 127);
  assert!(solution.is_some());
  solution = solve(13, 743);
  assert!(solution.is_some());
}

#[test]
fn test_possible_next() {
  let mut c = Chain::new();
  assert_eq!(vec![2], c.possible_next_values());
  c.add(2);
  assert_eq!(vec![3,4], c.possible_next_values());
  c.add(4);
  c.add(8);
  c.add(9);
  assert_eq!(vec![10, 11, 12, 13, 16, 17, 18], c.possible_next_values());
}  


#[test]
fn test_max_sum() {
  let mut c: Chain = Chain::new();
  assert_eq!(c.max_sum(2), 3); // 1 2
  assert_eq!(c.max_sum(5), 31); // 1 2 4 8 16
  c.add(2);
  c.add(3);
  assert_eq!(c.max_sum(5), 24); // 1 2 3 6 12
  c.add(4);
  assert_eq!(c.max_sum(6), 34); // 1 2 3 4 8 16
}

#[test]
fn test_min_sum() {
  let mut c: Chain = Chain::new();
  assert_eq!(c.min_sum(2), 3); // 1 2
  assert_eq!(c.min_sum(5), 15); // 1 2 3 4 5
  c.add(2);
  c.add(4);
  assert_eq!(c.min_sum(4), 12); // 1 2 4 5
  assert_eq!(c.min_sum(5), 18); // 1 2 4 5 6
}

#[test]
fn test_sum_n_2() {
  assert_eq!(sum_n_2(1), 1);
  assert_eq!(sum_n_2(2), 3);
  assert_eq!(sum_n_2(4), 10);
  assert_eq!(sum_n_2(10), 55);
}

#[test]
fn test_pow() {
  assert_eq!(pow(2, 0), 1);
  assert_eq!(pow(2, 1), 2);
  assert_eq!(pow(2, 8), 256);
}