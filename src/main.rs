#![allow(unused)]

use std::io;
use rand::Rng;

use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main_lesson_01() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
      .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn main_lesson_02() {
  const ONE_MIL: u32 = 1_000_000;
  const PI: f32 = 3.141592;
  let age = "47";
  let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number");
  age = age + 1;
  println!("I'm {} and I want want ${}", age, ONE_MIL);
}

fn main_lesson_03() {
  // Unsigned integer: u8, u16, u32, u64, u128, u256, usize
  // Signed integer: i8, i16, i32, i64, i128, isize
  println!("Max u32 : {}", u32::MAX);
  println!("Max u64 : {}", u64::MAX);
  println!("Max usize : {}", usize::MAX);
  println!("Max u128 : {}", u128::MAX);
  println!("Max f32 : {}", f32::MAX);
  println!("Max f64 : {}", f64::MAX);
}

fn main_lesson_04() {
  let is_true = true;
  let my_grade = 'A';
}

fn main_lesson_05() {
  let num_1: f32 = 1.111111111111111;
  println!("f32 : {}", num_1 + 0.111111111111111);
  let num_2: f64 = 1.111111111111111;
  println!("f64 : {}", num_2 + 0.111111111111111);
}

fn main_lesson_06() {
  let mut num_3: u32 = 5;
  let num_4: u32 = 4;
  num_3 += 1;
  println!("{} + 4 = {}", num_3, num_3 + num_4);
  println!("{} - 4 = {}", num_3, num_3 - num_4);
  println!("{} * 4 = {}", num_3, num_3 * num_4);
  println!("{} / 4 = {}", num_3, num_3 / num_4);
  println!("{} % 4 = {}", num_3, num_3 % num_4);
}

fn main_lesson_07() {
  let random_num = rand::thread_rng().gen_range(1..101);
  println!("Random : {}", random_num);
}

fn main_lesson_08() {
  let age: i32 = 8;
  if (age >= 1) && (age <= 18) {
    println!("Important Birthday");
  } else if (age == 21) || (age == 50) {
    println!("Important Birthday");
  } else if age >= 65 {
    println!("Important Birthday");
  } else {
    println!("Not an Important Birthday");
  }
}

fn main_lesson_09() {
  let mut my_age = 47;
  let can_vote = if my_age >= 18 {
    true
  } else {
    false
  };
  println!("Can Vote : {}", can_vote);
}

fn main_lesson_10() {
  let age2 = 8;
  match age2 {
    1..=18 => println!("Important Birthday"),
    21 | 50 => println!("Important Birthday"),
    65..=i32::MAX => println!("Important Birthday"),
    _ => println!("Not an Important Birthday"),
  };
}

fn main_lesson_11() {
  let my_age = 18;
  let voting_age = 18;
  match my_age.cmp(&voting_age) {
    Ordering::Less => println!("Can't vote"),
    Ordering::Greater => println!("Can vote"),
    Ordering::Equal => println!("You gained the right to vote"),
  };
}

fn main_lesson_12() {
  let arr_2 = [1,2,3,4,5,6,7,8,9];
  println!("1st : {}", arr_2[0]);
  println!("Length: {}", arr_2.len());
  
  let mut loop_idx = 0;
  loop {
    if arr_2[loop_idx] % 2 == 0 {
      loop_idx += 1;
      continue;
    }
    if arr_2[loop_idx] == 9 {
      break;
    }
    println!("Val : {}", arr_2[loop_idx]);
    loop_idx += 1;
  }
}

fn main_lesson_13() {
  let arr_2 = [1,2,3,4,5,6,7,8,9];
  let mut loop_idx = 0;
  while loop_idx < arr_2.len() {
    println!("Arr : {}", arr_2[loop_idx]);
    loop_idx += 1;
  }
}

fn main() {
  let arr_2 = [1,2,3,4,5,6,7,8,9];
  let mut loop_idx = 0;
}
