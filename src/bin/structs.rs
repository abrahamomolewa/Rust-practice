
use std::io;
use num::complex;
use std::cell::Cell;

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

struct Person {
    name: String,
    age: u32,

  }
  
  trait Human {
    fn new(name: String) -> Self;
    fn info(&self);
  }
  
  impl Human for Person {
    fn new(name: String) -> Person {
      Person { 
    name,
    age: 22,
    } 
  }
  
  fn info(&self) {
    println!("your name is {}, your age is {}", self.name, self.age);
  }
  }
  

  //Question 4 Create a generic struct for addition of numbers (they can be integer or float).
  struct  Addition<T> {
    num1: T,
    num2: T
}

//Question 5 Create an immutable struct with a mutable member.
#[derive(Debug)]
struct Persons {
    name: String,
     age: Cell<u32>
}


//Question 6:  Write a program to compare two dates entered by user. Make a structure named Date to store the elements day, month and year to store the dates. If the dates are equal, display "Dates are equal" otherwise display "Dates are not equal"    
struct Date {
  day: u32,
  month: u32,
  year: u32
}

fn compare_dates(date1: Date, date2: Date) -> bool {
  date1.year == date2.year && date1.month == date2.month && date1.day == date2.day
}
//

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


 let text: Person = Human::new("abraham".to_string());
      
 text.info();
//end of question 3 solution.

let add = Addition {
  num1: 4,
  num2: 5
};

println!("{}", add.num1 + add.num2);

let add2 = Addition {
  num1: 5.6,
  num2: 7.8
};
println!("{}", add2.num1 + add2.num2);

// end of question 4 solution.

let person = Persons {
  name: "Abraham".to_string(),
  age: Cell::new(43),
};
person.age.set(56);

println!("{:?}", person);

// end of question 5 solution 


//Body of question 6:

let mut date1 = Date { day: 0, month: 0, year: 0 };
let mut date2 = Date { day: 0, month: 0, year: 0 };

// Read the first date from the user.
println!("Enter the first date (dd mm yyyy):");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");
let mut input_parts = input.split_whitespace();
date1.day = input_parts.next().unwrap().parse().expect("Failed to parse day");
date1.month = input_parts.next().unwrap().parse().expect("Failed to parse month");
date1.year = input_parts.next().unwrap().parse().expect("Failed to parse year");

// Read the second date from the user.
println!("Enter the second date (dd mm yyyy):");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");
let mut input_parts = input.split_whitespace();
date2.day = input_parts.next().unwrap().parse().expect("Failed to parse day");
date2.month = input_parts.next().unwrap().parse().expect("Failed to parse month");
date2.year = input_parts.next().unwrap().parse().expect("Failed to parse year");

// Compare the two dates and print the result.
if compare_dates(date1, date2) {
    println!("Dates are equal.");
} else {
    println!("Dates are not equal.");
}

//end of question 6 body 
}



