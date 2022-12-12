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
fn print_str(x: String) {
    println! ("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println! ("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message:  {}", name);
}

fn main() {


    let str1= String::from("World");
    let str2 = str1.clone();
    //print_str(str1);
    let str3 = print_return_str(str1);
    println!("str3 =  {}", str3);
    //print_str(str3);
    let mut str4 = String::from("Julio");
    change_string(&mut str4);
}
 

*/


/*
use std::collections::HashMap;


fn main() {

    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k,v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!(" Length : {}", heroes.len());
    
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some (x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),

        }
    }

}

*/


/*
fn main() {

    struct Customer {
        name: String,
        address: String,
        balance: f32
     }

    let mut zag = Customer {
        name: String::from("Zag Guedes"),
        address: String::from("354 Zag´s Street"),
        balance: 569.00,
    };
    println!("Zag address {}", zag.address);
    zag.address = String::from("Other Zag´s address");
    println!("Zag address {}", zag.address);
}
 

*/


/*

fn main() {

    struct Rectangle<T, U> {
        length: T,
        height: U,
     }

    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };
}

*/


/*
fn main() {

    const PI: f32 = 3.141592;
  
    trait Shape{
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
            return &self.length * &self.width;
        }
    }
     
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (&self.length/2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());
   
}
*/


// Crates: Modules that produce a library or an executable
// Modules: Organize and handle privacy
// Packages:  Build, test and share crates
// Path: A way of naming an item such as a struct or a  function 


/*

mod restaurant;
use crate::restaurant::order_food;

fn main() {

    order_food();

}

*/

/*
fn main() {

    panic!("Terrible Error");

}

Running `target\debug\rust_tutorial.exe`
thread 'main' panicked at 'Terrible Error', src\main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library\std\src\panicking.rs:584
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library\core\src\panicking.rs:142
   2: rust_tutorial::main
             at .\src\main.rs:16
   3: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/897e37553bba8b42751c67658967889d11ecd120\library\core\src\ops\function.rs:248
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\rust_tutorial.exe` (exit code: 101)
*/

/*
fn main() {

    let lil_arr = [1,2];
    println!("{}", lil_arr[10]);

}

error: this operation will panic at runtime
  --> src\main.rs:17:20
   |
17 |     println!("{}", lil_arr[10]);
   |                    ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 10
   |
   = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `rust_tutorial` due to previous error

*/

/*

fn main() {

    let path = "lines.txt" ;
    let output = File::create(path);
    //Result is an enum and has 2 variants  Ok and Err
    // enum Result<T,E> {
    //   Ok(T),
    //   Err(E),
    //}
    // where T is the data typeof the value returned
    // and E is the type of Error
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}" , error),

    };
    write!(output, "Just some\nrandom words").expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("Rand.txt");
    let  output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("Rand.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Can´t create file: {:?}", e),
                },
                _other_error => panic!("Problem openning the file: {:?}", error),
        },
    };

}

*/

/*
fn main() {

    let mut arr_it = [1,2,3,4];
    //val are borrowed, the collectio is still in memory, but values cannot be changed
    for val in arr_it.iter(){
        println!("{}", val);
    }
    //the collection is consumed but the collection will not be longer there to be used
    //arr_it.into_iter();

    let mut iter1 = arr_it.iter();
    println!("1st value: {:?}", iter1.next());

}

*/

/*

fn main() {

    // Closure is a function without a name 
    //and they are likely stored on a variable 
    //and they can be used to pass a function into another function
    // Closures can access variable outside their body
    // let var_name = |parameters| -> return type {BODY}

    let can_vote = |age: i32| { 
        age>=18 
    };
    println!("Can vote: {}", can_vote(8));

}
 
*/

/*

fn main() {

   let mut samp1 =5;
   let print_var = || println!("Sample 1: {}", samp1);
   print_var();
   samp1 = 10;
   let mut change_var = || samp1 += 1;
   change_var();
   println!("Sample 1: {}", samp1);
   samp1 = 10;
   println!("Sample 1: {}", samp1);

}

*/

/*

fn main() {

   fn use_func<T> (a: i32, b: i32, func: T) -> i32
   where T: Fn(i32, i32) -> i32 {
        func(a,b)
   } 

   let sum = |a,b| a+b;
   let product = |a,b| a*b;

   println!(" 5 + 4 = {}", use_func(5, 4, sum));
   println!(" 5 * 4 = {}", use_func(5, 4, product));

}

*/

/*

    // A pointer is an address to a location in memory
    // Smart pointers is a pointer that besides it owns the data it has functions for manipulating that data,
    // Smart pointers just provide functionality beyond referencing a specific location in mmemory 
    // and they can be used to track the ownership of data
    //    box smart pointer -- just stores data on the heap instead of the stack 
    //    reference pointer
    // reference operator & to borrow a value  rather then taking it and having it cleaned out of memory
    // Strings and Vectors are also smart pointers
    // BOX is normally going to be used when you have a large amount of data that is stored on the heap 
    // and then you pass pointers to it on the stack
    fn main() {
    
    let b_int = Box::new(10);
    println!("b_int = {}", b_int);
    
    }

*/


/*

fn main() {
    
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T>{
        pub fn new(key: T) -> Self {
            TreeNode { 
                left: None, 
                right: None,
                key, 
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
     .left(TreeNode::new(2))
     .right(Treenode::new(3));

}

*/


/*
 
// A thread is going to handle the scheduling as well as the execution of the blocks of code that will run in parallel independently
// common problems with parallel programing:
//      threads are accessing data in the wrong order
//      threads are blocked from executing because of confusion over requirements to proceed with the execution

use std::thread;
use std::time::Duration;



fn main() {
    
    thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //there are no guarantees on whenthreads will execute
    // and if they will even complete execution 
}

*/


/*
use std::thread;
use std::time::Duration;


fn main() {
    
   let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //there are no guarantees on whenthreads will execute
    // and if they will even complete execution 

    //to guarantee they execute till the end you have to join them
    thread1.join().unwrap();

}

*/

/*

use std::thread;
use std::time::Duration;

fn main() {
   
    pub struct Bank{
        balance: f32,
    }

    pub fn withdraw(the_bank: &mut Bank, amount: f32){
        the_bank.balance -= amount;

    }

    let mut bank = Bank {balance: 100.0, };
    withdraw(&mut bank,5.0);
    println!("Balance: {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.0);
    }

    thread::spawn(|| {
        customer(&mut bank);
    })
    .join().unwrap();
   
}

*/


/*
// A smart pointer will allow multiple owners and it will block access when needed
//std::sync::Arc
//Arc stands for Atomically referenced counted 
//&Arc -- a pointer using this will be thread-safe referenced counting pointer
//The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. 
//Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap 
//as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, 
//the value stored in that allocation (often referred to as "inner value") is also dropped.


//Mutex will block threads that are waiting for lock to be available
*/
/*


*/
/*


*/
/*


*/
/*


*/