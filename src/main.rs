#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin().read_line((&mut name))
    //     .expect("Didn't receive Input");

    // println!("Hello {}!, {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_00_000;
    // const PI: f32 = 3.141592;
     
    // let age: &str = "47 ";
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // let age2 = 20;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX => println!("Important Birthday"),
    //     _ => println!("Not an important birthday"),
    // };

    // let my_age = 18;
    // let voting_age = 19;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Can't"),
    //     Ordering::Greater => println!("Can"),
    //     Ordering::Equal => println!("Yes you can")
    // }

    // let arr_1 = [1,2,3,4,5,6,7,8,9];
    // println!("1st {}", arr_1[0]);
    // println!("length {}", arr_1.len());

    // let mut loop_idx = 0;
    // loop {
    //     if arr_1[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_1[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("Val: {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    // loop_idx = 0;
    // while loop_idx < arr_1.len() {
    //     println!("Arr: {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    // for val in arr_1.iter() {
    //     println!("val: {}", val);
    // }
    
    // let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    // println!("Name:  {}", my_tuple.1);
    // let(v1, v2, v3) = my_tuple;

    // let st3 = String::from("x r t b h k k a m c");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }
    // let st4 = "Random String";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6]; //slicing
    // println!("String lenght {}", st6.len());
    // st5.clear();

    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7; // st6 does not exist at this point anymore and become st6

    // for char in st8.bytes() {
    //     println!("{}", char)
    // }

    


}
