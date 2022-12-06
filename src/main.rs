#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


use std::ops::Add;


fn get_sum_gen<T: Add<Output = T>> (x: T, y: T) -> T {
    return x+y;
}

fn main() {


    let str1= String::from("World");
    let str2 = str1;
    println!("Hello {}", str1);
    

    println!("5 + 4 = {}", get_sum_gen(5,4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));
}
 