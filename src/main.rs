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

fn main() {
}