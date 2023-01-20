#![allow(unused)]

use std::io;
use std::process::Output;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::{hash_map, HashMap};

mod restaurant;
use crate::restaurant::order_food;

mod binary_tree;

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_string(x: String) -> String {
    println!("A string {}", x);
    return x;
}

fn change_string(name: &mut String){
    name.push_str(" is happy ");
    println!("Message: {}", name);
}


fn main() {
    type BinaryTree = binary_tree::binary_tree::TreeNode<i32>;

    let node1 = BinaryTree::new(1)
        .right(BinaryTree::new(3))
        .left(BinaryTree::new(2));

    node1.in_order();

    // order_food();

    // struct Rectangle <T, U> {
    //     length: T, 
    //     height: U,
    // }

    // let rec = Rectangle{
    //     length: 4, height: 10.5
    // };

    // trait Shape {
    //     fn new(length: f32, width: f32) -> Self;
    //     fn area(&self) -> f32;

    // }
    // struct Rectangle {length: f32, width: f32};
    // struct Circle {length: f32, width: f32};

    // impl Shape for Rectangle {
    //     fn new(length: f32, width: f32) -> Rectangle {
    //         return Rectangle{length, width};
    //     }
    //     fn area(&self) -> f32 {
    //         return self.length * self.width;
    //     }
    // }

    // impl Shape for Circle {
    //     fn new(length: f32, width: f32) -> Circle {
    //         return Circle{length, width};
    //     }
    //     fn area(&self) -> f32 {
    //         return (self.length / 2.0).powf(2.0) * 3.141592;
    //     }
    // }

    // let rec: Rectangle = Shape::new(10.0, 10.0);
    // let circ: Circle = Shape::new(10.0, 10.0);
    // println!("Rec Area: {}", rec.area());
    // println!("Circ Area: {}", circ.area());

    // struct Customer {
    //     name: String,
    //     address: String, 
    //     balance: f32,
    // }

    // let mut bob = Customer{
    //     name: String::from("Bob Smith"),
    //     address: String::from("555 Main st"),
    //     balance: 234.50
    // };

    // bob.address = String::from("505 Main st");
    // println!("Bob address is {}", bob.address );

    // let mut heroes = HashMap::new();
    // heroes.insert("Superman", "Clark Kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("Flash", "Barry Allen");

    // for (k, v ) in heroes.iter() {
    //     println!("{} = {}", k, v);
    // }

    // println!("Length: {}", heroes.len());
    // println!("Is batman a heroes: {}", heroes.contains_key(&"Batman"));
    // println!("Superman is {}", heroes["Superman"]);


    // let mut str1 = String::from("World");
    // // let str2 = str1.clone();
    // // println!("Hello {}", str1);

    // // print_str(str1);
    // // let str3 = print_return_string(str1);
    // // println!("{}", str3);
    // change_string(&mut str1);


    // println!("5 + 4 = {}", get_sum_gen(5, 4));
    // println!("3.2 + 4.6 = {}", get_sum_gen(3.2, 4.6));

    // let num_list = vec![1,2,3,4,5];
    // println!("Sum of list = {}", sum_list(&num_list))

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
