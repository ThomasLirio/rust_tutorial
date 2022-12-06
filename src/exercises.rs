#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    /* 
    println!("what is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Don't receive input");
    println!("Hello {}!, {}", name.trim_end(),greeting);
    */

    /* 
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592; 
    let age: &str = "34";
    let mut age: u32 =age.trim().parse()
        .expect("Age was not assigned a number");
    age = age -1;
    println!("I'm {} and i want €{}", age, ONE_MIL);
    */

    /* 
    println!("MAX: u32 {}", u32::MAX);
    println!("MAX: u64 {}", u64::MAX);
    println!("MAX: usize {}", usize::MAX);
    println!("MAX: u128 {}", u128::MAX);
    println!("MAX: f32 {}", f32::MAX);
    println!("MAX: f64 {}", f64::MAX);

    let is_true=true; 
    let my_grade ='A';

    let num_1 : f32 = 1.1111111111111111;
    println!("f32 {}", num_1 + 0.1111111111111111);
    let num_2 : f64 = 1.1111111111111111;
    println!("f64 {}", num_2 + 0.1111111111111111);
    */

    /* 
    let mut num_3: u32 = 5;
    let num_4: u32 = 3;
    println!("5 + 3 = {}", 5 + 3);
    println!("5 - 3 = {}", 5 - 3);
    println!("5 * 3 = {}", 5 * 3);
    println!("5 / 3 = {}", 5 / 3);
    println!("5 % 3 = {}", 5 % 3);
    
    num_3 += 1;
    */

    /*
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random {}", random_num);
    */

    /*
    let age =8;

    if (age >= 1 && age <=18) {
        println!("Important birthday 1");
    } else if (age == 21 || age==50) {
        println!("Important birthday 2");
    } else if (age>= 65) {
        println!("Important birthday 3");
    } else {
        println!("Not so important birthday");
    }
     */

     /* 
     let my_age = 34;
     let mut can_vote = my_age >=18;
     println!("Can vote : {}", can_vote);
    */

    /* 
    let age_2 = 21;
    match age_2 {
        1..=18 =>  println!("Important birthday 1"),
        21 | 50 =>  println!("Important birthday 2"),
        65..=i32::MAX => println!("Important birthday 3"),
        _ =>  println!("Not so important birthday"),
    };
    */

    /* 
    let my_age = 21;
    let voting_age = 21;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Equal => println!("You gain the right to vote"),
        Ordering::Greater => println!("Can vote"),
    };
    */

    /* 
    let arr_1 = [1,2,3,4];
    println!("1st {}", arr_1[0]);
    println!("length {}", arr_1.len());
    */

    /*

    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx: usize = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue
        } 
        if arr_2[loop_idx] == 9 {
            break;
        }
       
        loop_idx += 1;

    }
    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("arr:  {}", arr_2[loop_idx]);
        loop_idx += 1;
    }    
    
    for val in arr_2.iter() {
        println!("val:  {}", val);
    }
    */

    /* 
    let my_tuple : (u8, String, f64) = (34, "Asz".to_string(), 50_000_000.00);
    println!("Name:  {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age:  {}", v1);
    */

    /* 
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    st1.push('Z');
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
    */

    /*
    let st3 = String::from("x r t b h k k a m c");
    println!("{}", st3);
    let mut v1: Vec<char> = st3.chars().collect();
    for char in &v1 {
        println!("{}", char);
    }
    v1.sort();
    for char in &v1 {
        println!("{}", char);
    }
    v1.dedup();
    for char in &v1 {
        println!("{}", char);
    }
    let str4: &str = "Random string";
    let mut st5: String = str4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    for byte in byte_arr1 {
        println!("{}", byte);
    }
    let st6 = &st5[0..6];
    for char in st6.to_string().chars() {
        println!("{}", char);
    }

    println!("String Length: {}", st6.len());
    st5.clear();
    println!(" np st5 it was cleared {}", st5);

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    println!("{}", st8);

    for char in st8.bytes() {
        println!("{}", char);
    }
     */

    /*
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{} + {} = {}", int_u8, int2_u8, int3_u32);
    */

    /*
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }
    
    let today: Day = Day::Sunday;
    
    match today {
        Day::Monday => println!("Blue Monday"),
        Day::Tuesday => println!("Feira da Ladra"),
        Day::Wednesday => println!("Geni e o Zepplin"),
        Day::Thursday => println!("Halloween"),
        Day::Friday => println!("I am in Love"),
        Day::Saturday => println!("Porque hoje é Sábado"),
        Day::Sunday => println!("A construcao"),
    }

    println!("Is today weekend {}", today.is_weekend());
    */

    /*
     let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st {}", vec2[0]);

    let second = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;

    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
    */

    

   
}

/*
fn say_hello() {
    println!("Hello");
}

fn main() {
   
   say_hello();

} 
*/

 /*
fn get_sum(x: i32, y:i32) {
    println!("{} + {} = {}" , x,y,x+y);
}

fn main() {
   
    get_sum(5,4);

}
*/

/*
fn get_sum_2(x: i32, y:i32) -> i32 {
    x+y
}

fn main() {
    println!("{} + {} = {}" , 5,4,get_sum_2(5,4));
}
*/


/*
fn get_sum_3(x: i32, y:i32) -> i32 {
   return x+y;
}

fn main() {
    println!("{} + {} = {}" , 5,4,get_sum_3(5,4));
}
*/

/*
fn get_2(x: i32) -> (i32,i32) {
   return (x+1, x+2);
}

fn main() {

    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {}", val_1, val_2);
}
 
*/

/*
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
   sum
}

fn main() {

    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("sum of list: {}", sum_list(&num_list));
}
*/

/*
use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>> (x: T, y: T) -> T {
    return x+y;
}

fn main() {

    println!("5 + 4 = {}", get_sum_gen(5,4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));
}
 */

/*
// Stack :  stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you request a certain amount of space
// The Operating System finds space available and returns an address for that space called pointer

// RULES:
    // 1 - Each value has a variable that is called its owner
    // 2 - There is only a owner at a time 
    // 3 - When the owner goes out of scope the value disappears    
    
    this code gives an error
    fn main() {
        let str1= String::from("World");
        let str2 = str1;
        println!("Hello {}", str1);
    }

    error[E0382]: borrow of moved value: `str1`
  --> src\main.rs:22:26
   |
20 |     let str1= String::from("World");
   |         ---- move occurs because `str1` has type `String`, which does not implement the `Copy` trait
21 |     let str2 = str1;
   |                ---- value moved here
22 |     println!("Hello {}", str1);
   |                          ^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.

str1 does not exists at the time of println!
the value stored in str1 has a new owner that is str2

String does not implement the copy, when a structure implement the copy the attribution of its value to another variable creates a copy 
otherwise just the owner is changed and because str1 has no owner is released from the memory

this code will not generate an error
    fn main() {
        let str1= String::from("World");
        let str2 = str1.clone();
        println!("Hello {}", str1);
    }


*/

/*

*/

/*

*/