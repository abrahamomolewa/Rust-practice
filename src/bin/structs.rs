
use std::io;
use num::complex;

//question 1: Write a program to store and print the roll no., name , age and marks of a student using structures.
struct Data {
    roll_no: u32,
    name: String,
    age: u32,
    mark_of_student: u32,
}

//Question 2: Write a program to add, subtract and multiply two complex numbers using structures to function.

struct Complex<T> {
    real: T,
    img:  T,
}

//question 3  Replicate constructor function with name new() returning type Self.

fn main(){

      let mut name1 = String::new(); 
      println!("what is your name?");
      io::stdin().read_line(&mut name1).expect("type your  name");
       
    let mut rollno = String::new();
    let mut age_input  = String::new();
    let mut mark_of_student_input = String::new();

    let roll_no1: u32 = loop {
         rollno.clear(); // clear input so that we don't try parsing the old user input as well
    
        println!("What your roll number:");
        io::stdin().read_line(&mut rollno).expect("reason it should not fail");
    
        match rollno.trim().parse() {
            Ok(roll_no1) => break roll_no1,
            Err(_) => println!( "What's your roll number:")
        };
    };

    let age1: u32 = loop {
          age_input.clear();

          println!("what's your age:");
          io::stdin().read_line(&mut age_input).expect("it shouldn't fail");
           
          match age_input.trim().parse() {
            Ok(check) => break check,
            Err(_)  => println!("What's your age:")
          };
    }; 


    let mark_of_student1: u32 = loop {
        mark_of_student_input.clear();
        println!("what your exam score:");
        io::stdin().read_line(&mut mark_of_student_input).expect("it shouldn't fail");

        match mark_of_student_input.trim().parse() {
            Ok(check)  => break check,
            Err(_)  => println!("what's your exam score")
        };

    };
    
     
    let data = Data {
        roll_no: roll_no1,
        name: name1,
        age: age1,
        mark_of_student: mark_of_student1,
    };
    println!("Your roll number is {},\nyour name is {}your age is {},\nand the mark of the student is {}", data.roll_no, data.name, data.age, data.mark_of_student );
//end of question 1 solution.



    let real = complex::Complex::new(10.0, 20.1);
      let img =  complex::Complex::new(10.1, 20.1);

    let complex = Complex {
         real: real,
         img:  img,
    };

    let sum = complex.real + complex.img;

    println!("{}", sum); 
 //end of question 2 solution.


}

