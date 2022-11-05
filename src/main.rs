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

fn main() {
  const ONE_MIL: u32 = 1_000_000;
  const PI: f32 = 3.141592;
  let age = "47";
  let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number");
  age = age + 1;
  println!("I'm {} and I want want ${}", age, ONE_MIL);
}