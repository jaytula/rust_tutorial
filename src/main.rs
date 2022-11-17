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

fn main_lesson_14() {
  let arr_2 = [1,2,3,4,5,6,7,8,9];
  let mut loop_idx = 0;
  for val in arr_2.iter() {
    println!("Val : {}", val);
  }
}

fn main_lesson_15() {
  let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
  println!("Name : {}", my_tuple.1);
  let (v1, v2, v3) = my_tuple;
  println!("Age : {}", v1);
}

fn main_lesson_16() {
  let mut st1 = String::new();
  st1.push('A');
  st1.push_str(" word");
  for word in st1.split_whitespace() {
    println!("{}", word);
  }

  let st2 = st1.replace("A", "Another");
  println!("{}", st2);  
}

fn main_lesson_17() {
  let st3 = String::from("x r t b h k k a m c");
  let mut v1: Vec<char> = st3.chars().collect();
  v1.sort();
  v1.dedup();
  for char in v1 {
    println!("{}", char);
  }
  let st4: &str = "Random string";
  let mut st5: String = st4.to_string();
  println!("{}", st5);
  let byte_arr1 = st5.as_bytes();
  let st6 = &st5[0..6];
  println!("String length : {}", st6.len());
  st5.clear();

  let st6 = String::from("Just some");
  let st7 = String::from(" words");
  let st8 = st6 + &st7;
  for char in st8.bytes() {
    println!("{}", char);
  }
}

fn main_lesson_18() {
  let int_u8: u8 = 5;
  let int2_u8: u8 = 4;
  let int3_u32: u32 = int_u8 as u32 + int2_u8 as u32;
  println!("{}", int3_u32);
}

fn main_lesson_19() {
  enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
  }

  impl Day {
    fn is_weekend(&self) -> bool {
      match self {
        Day::Saturday | Day::Sunday => true,
        _ => false
      }
    }
  }

  let today = Day::Monday;
  match today {
    Day::Monday => println!("Everyone hates Monday"),
    Day::Tuesday => println!("Donut day"),
    Day::Wednesday => println!("Hump day"),
    Day::Thursday => println!("Pay day"),
    Day::Friday => println!("Almost weekend"),
    Day::Saturday => println!("weekend"),
    Day::Sunday => println!("weekend"),
  }

  println!("Is today the weekend {}", today.is_weekend());
}

fn main_lesson_20() {
  let vec1: Vec<i32> = Vec::new();
  let mut vec2 = vec![1,2,3,4];
  vec2.push(5);
  println!("1st : {}", vec2[0]);
  let second: &i32 = &vec2[1];
  match vec2.get(1) {
    Some(second) => println!("2nd : {}", second),
    None => println!("No 2nd value"),
  }

  for i in &mut vec2 {
    *i *= 2;
  }
  for i in &vec2 {
    println!("{}", i);
  }
  println!("Vec Length {}", vec2.len());
  println!("Pop: {:?}", vec2.pop());
}

fn say_hello() {
  println!("Hello");
}

fn main_lesson_21() {
  say_hello();
}

fn get_sum(x: i32, y: i32) {
  println!("{} + {} = {}", x, y, x+y);
}

fn main_lesson_22() {
  get_sum(5, 4);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
  x + y
}

fn main_lesson_23() {
  println!("{}", get_sum_2(5, 7));
}

fn get_sum_3(x: i32, y: i32) -> i32 {
  return x + y;
}

fn main_lesson_24() {
  println!("{}", get_sum_3(6, 7));
}

fn get_2(x: i32) -> (i32, i32) {
  return (x+1, x+2);
}
fn main_lesson_25() {
  let (val_1, val_2) = get_2(7);
  println!("Nums : {} {}", val_1, val_2);
}

fn sum_list(list: &[i32]) -> i32 {
  let mut sum = 0;
  for &val in list.iter() {
    sum += &val;
  }
  sum
}

fn main_lesson_26() {
  let num_list = vec![1,2,3,4,5];
  println!("Sum of list = {}", sum_list(&num_list));
}

use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
  return x + y;
}

fn main_lesson_27() {
  println!("5 + 4 = {}", get_sum_gen(5, 4));
  println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
}

/*  
  Stack : Stores values in a last in first out format
  Heap : When putting data on the heap you request a certain amount
    of space. The OS finds space available and returns an address
    for that space called a pointer.

  RULES:
  1. Each value has a variable that's called its owner
  2. There is only one owner at a time
  3. When the owner goes out of scope the value disappears
*/

fn print_str(x: String) {
  println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
  println!("A string {}", x);
  x
}

fn change_string(name: &mut String) {
  name.push_str(" is happy");
  println!("Message : {}", name);
}

fn main_lesson_28() {
  let mut str1 = String::from("Derek");
  change_string(&mut str1);
  println!("str is {}", str1);
}

use std::collections::HashMap;

fn main_lesson_29() {
  let mut heroes = HashMap::new();
  heroes.insert("Superman", "Clark Kent");
  heroes.insert("Batman", "Bruce Wayne");
  heroes.insert("Flash", "Barry Allen");

  for(k, v) in heroes.iter() {
    println!("{} = {}", k, v);
  }
  println!("Length : {}", heroes.len());

  if heroes.contains_key(&"Batman") {
    let the_batman = heroes.get(&"Batman");
    match the_batman {
      Some(x) => println!("Batman is a hero"),
      None => println!("Batman is not a hero"),
    }
  }
}

fn main_lesson_30() {
  struct Customer {
    name: String,
    address: String,
    balance: f32,
  }

  let mut bob = Customer{
    name: String::from("Bob Smith"),
    address: String::from("555 Main St"),
    balance: 234.50
  };

  bob.address = String::from("505 Main St");
}

fn main_lesson_31() {
  const PI: f32 = 3.14159;
  trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
  }
  struct Rectangle {length: f32, width: f32};
  struct Circle {length: f32, width: f32};

  impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
      return Rectangle { length, width };
    }
    fn area(&self) -> f32 {
      return self.length * self.width;
    }
  }

  impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
      return Circle { length, width };
    }
    fn area(&self) -> f32 {
      return (self.length / 2.0).powf(2.0) * PI;
    }
  }

  let rec: Rectangle = Shape::new(10.0, 10.0);
  let circ: Circle = Shape::new(10.0, 10.0);

  println!("Rec Area : {}", rec.area());
  println!("Circle Area : {}", circ.area());
}

// Crates : Modules that produce a library or executable
// Modules : Organize and handle privacy
// Packages: Build, test and share crates
// Paths : A way of naming an item such as a struct, function

mod restaurant;
use crate::restaurant::order_food;

fn main_lesson_32() {
  order_food();
}

fn main_lesson_33() {
  panic!("Terrible Error");
}

fn main_lesson_34() {
  let lil_arr = [1,2];
  println!("{}", lil_arr[10]);
}

/* Result has 2 variants Ok and Err
   enum Result<T, E> {
    Ok(T),
    Err(E),
   }
   // Where T represents the data typeof the value and returns and E
   // the type of error
*/
  

fn main_lesson_35() {
  let path = "lines.txt";
  let output = File::create(path);
  let mut output = match output {
    Ok(file) => file,
    Err(error) => {
      panic!("Problem creating file : {:?}", error);
    }
  };
  write!(output, "Just some\nRandom words").expect("Failed to write to file");
  
  let input = File::open(path).unwrap();
  let buffered = BufReader::new(input);
  for line in buffered.lines()
  {
    println!("{}", line.unwrap());
  }

  let output2 = File::create("rand.txt");
  let output2 = match output2 {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("rand.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Can't create file {:?}", error),
      },
      _other_error => panic!("problem opening file : {:?}", error),
    },
  };
}

fn main_lesson_36() {
  let mut arr_it = [1,2,3,4];
  for val in arr_it.iter() {
    println!("{}", val);
  }
  // arr_it.into_iter()
  let mut iter1 = arr_it.iter();
  println!("1st : {:?}", iter1.next());
}

fn main_lesson_37() {
    // Closures...
    // let var_name = |parameters| -> return_type {BODY}

    let can_vote = |age: i32| {
      age >= 18
    };

    println!("Can vote : {}", can_vote(8));
}

fn main_lesson_38() {
  let mut samp1 = 5;
  let print_var = || println!("samp1 = {}", samp1);
  print_var();
  samp1 = 10;
  let mut change_var = || samp1 += 1;
  change_var();
  println!("samp1 = {}", samp1);
  samp1 = 10;
  println!("samp1 = {}", samp1);
}

fn main() {
}