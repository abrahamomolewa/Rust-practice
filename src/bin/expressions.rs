
use std::io;
use std::{i32};


//question 3: Using a match statement to print one for input 1, two for 2 and so on, and NaN on default.
fn in_word(num: i32) -> String {

    match num {
      9 => "nine".to_string(),
      8 => "eight".to_string(),
      7 => "seven".to_string(),
      6 => "six".to_string(),
      5 => "five".to_string(),
      4 => "four".to_string(),
      3 => "three".to_string(), 
      2 => "two".to_string(),
      1 => "one".to_string(),
      _ => "out of scope".to_string()
    }
}

//Question 4: Find and print factorial of a program using recursion.

fn factorial(num: u128) -> u128 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num
    }
}

//Qestion 1: Write a function to swap 2 numbers.
fn swap(a: i32, b: i32) -> (i32, i32) {
   (b, a)
}
//end of question 1



//question 2: Write a program to find prime numbers upto a number N
fn primes(max: u32) -> Vec<u32> {
    let mut primes = vec![2];
    for n in 3..max {
        if primes.iter().all(|p| n % p != 0) {primes.push(n);}
    }
    primes
}

//Question 10: Print the following pattern using loops.


   /* *
     ***
    *****
   *******
  ********* */
  fn print_pattern() {
    for i in 0..5 {
        // Print the leading spaces.
        for _ in 0..(5 - i - 1) {
            print!(" ");
        }

        // Print the stars.
        for _ in 0..(2 * i + 1) {
            print!("*");
        }

        // Move to the next line.
        println!();
    }
}
///


fn main() {

//Question 5: create a loop
'outer: for i in 0..5 {
     for w in 0..5 {
        println!("{}, {}", i, w);
        if w  == 4 {
            break 'outer;
        }
    }
}

//end of question 5

//Question 6: Take a integer input from the user and print multipication table for that integer using a loop.
println!("type a number to print multipication  table:");
let mut number = String::new();
io::stdin().read_line(&mut number).expect("Couldn’t read from stdin");

for i in 0..12 {
    match number.trim_end().parse::<i32>() {
        Ok(done) => println!("{} * {} = {}", done, i, done * i),
        Err(_) => println!("invalid")
    }
}
//end of question 6

//Queesion 7: Check whether the input number is odd or even and print odd or even respectively.
println!("please enter a number to check if odd or even:");
let mut number = String::new();
io::stdin().read_line(&mut number).expect("nan");

match number.trim().parse::<i32>() {
    Ok(done) => if done % 2 == 0 { println!("{done} is an even number ")} else {println!("{done} is an odd number "); },
    Err(_)   => println!("invalid")
}
//end of question 7

//Question 4: body
println!("please type a number to print a factorial:");
let mut number = String::new();
io::stdin().read_line(&mut number).expect("NAN");

match number.trim().parse::<u128>() {
    Ok(done) =>  println!("{}",factorial(done)),
    Err(_)  => println!("invalid")
};
//end of question 4



//Question 3 body
println!("println a number you want in words:");
let mut number = String::new();
io::stdin().read_line(&mut number).expect("NAN");

match number.trim().parse::<i32>(){
    Ok(done) => println!("{}", in_word(done)),
    Err(_)  =>  println!("NAN")
}
//end of question 3


//Question 8: Create a program to check for leap year.
//note: 1900 is not a leap year. 

println!("Type a year to know if leap year or not:");
let mut year = String::new();
io::stdin().read_line(&mut year).expect("nan");

match year.trim().parse::<i32>() {
    Ok(year) => if year % 400 == 0 || year % 4 == 0 { println!(" {} is a leap year", year)} else { println!("{} is not a leap year", year)},
    Err(_)   => println!("NAN")

}
//end of question 8



//body of question 2
println!("Type a number to print out the primes:");
let mut prime = String::new();
io::stdin().read_line(&mut prime).expect("NAN");

match prime.trim().parse(){
    Ok(prime_num) => println!("{:?}", primes(prime_num)),
    Err(_)   => println!("NAN")
}
//end of question 2

//Question 9: WAP to iterate over 2 vectors at once. hint: try using .zip method.
println!(" show how to iterate two vectors at once");
let vec1 = vec![1,3,5,7,9];
let vec2  = vec![2,4,6,8,10];

let zipit = vec1.iter().zip(vec2.iter());

for (vec1, vec2) in zipit {
    println!("{}, {}", vec1, vec2)
}
//end of question 9

//body of question 10
print_pattern();
//end of question 10 body

}    