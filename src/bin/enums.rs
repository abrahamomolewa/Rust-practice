use std::io;
use std::collections::HashMap;


//question 1: Create an enum that contains HTTP 4xx Client Errors i.e. Not Found Should have value 404 associated with it.
enum ClientErrors {
    BadRequest,
    Unauthorized,
    Forbidden,
    PageNotFound 
 }
 
 fn match_errors(error: ClientErrors) -> u32 {
     match error { 
      ClientErrors::BadRequest => 400,
      ClientErrors::Unauthorized => 401,
      ClientErrors::Forbidden => 403,
      ClientErrors::PageNotFound => 404
     }
 }
 
 fn printerror() {
       
     println!("{}", match_errors(ClientErrors::BadRequest));
     println!("{}", match_errors(ClientErrors::Unauthorized));
     println!("{}", match_errors(ClientErrors::Forbidden));
     println!("{}", match_errors(ClientErrors::PageNotFound));
 
 }
 //////
 
 //Question 2: Write an enum to store information of whether a person is a child or adult based on his/her age.
 enum Person {
    AdultOrNot(i32),

}
///


//Question 4; 
#[allow(dead_code)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}
//end of question 4

//Question 3: Use pattern matching to associate a enum of Grade type with a student based on his/her marks
#[derive(Debug)]
enum Grade {
    A,
    B,
    C,
    F
}
/// 

//Question 5: Use pattern matching to find that whether a point lies on X-Axis, Y-Axis or on which quadrant.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}
#[derive(Debug)]
enum PointLocation {
    OnXAxis(f64),
    OnYAxis(f64),
    OnOrigin,
    InQuadrant(i32)
}

fn classify_point(point: Point) -> PointLocation {
    if point.x == 0.0 && point.y == 0.0 {
        PointLocation::OnOrigin
    } else if point.x == 0.0 {
        PointLocation::OnYAxis(point.y)
    } else if point.y == 0.0 {
        PointLocation::OnXAxis(point.x)
    } else if point.x > 0.0 && point.y > 0.0 {
        PointLocation::InQuadrant(1)
    } else if point.x < 0.0 && point.y > 0.0 {
        PointLocation::InQuadrant(2)
    } else if point.x < 0.0 && point.y < 0.0 {
        PointLocation::InQuadrant(3)
    } else {
        PointLocation::InQuadrant(4)
    }
}

///


#[derive(Debug)]
struct  InputError;

impl  Grade  {
    fn get_str(s: u32) -> Result<Grade, InputError> {
      match s {
        80..=100 => Ok(Grade::A),
        70..=79  => Ok(Grade::B),
        60..=69  => Ok(Grade::C),
        0..=59   => Ok(Grade::F),
        _        => Err(InputError)
      }
    }
    fn enum_match(grade: Grade)  {
        match grade {
            Grade::A => println!("YOiu got grade A"),
            Grade::B => println!("YOu got grade B"),
            Grade::C => println!(" YOu got grade C"),
            Grade::F => println!("You got grade F"),
        }
    }
}
////
 
 fn main() {
    //question 1: body
     printerror();
     //end of question 1


     // body of question 2
     let mut age = String::new();

     let input: i32 = loop {
      age.clear();
 
      println!("Type your age:");
 
      io::stdin().read_line(&mut age).expect("expecting an age");
 
      match age.trim().parse() {
         Ok(check) => break check,
         Err(_)  => println!("type a numeric age")
      }
     };
   
 let adult = Person::AdultOrNot(input);
 
 match adult  {
     Person::AdultOrNot(age) => if age < 18 {println!("you are a child")} else {println!("you are grown")},
 };
// end of question 2


//Body of question 3
let mut grade =String::new();
       
let input: u32  = loop {
    grade.clear();
    println!("Type your score");
    io::stdin().read_line(&mut grade).unwrap();
     match grade.trim().parse() {
      Ok(done) => break done,
      Err(_) => println!("type your fuxking Score"),
     }
};


let dir = Grade::get_str(input).unwrap();

Grade::enum_match(dir);
//end of question 3

//body of question 5
// Example usage:
let point1 = Point { x: 0.0, y: 0.0 };
println!("{:?}", classify_point(point1));

let point2 = Point { x: 3.0, y: 0.0 };
println!("{:?}", classify_point(point2));

let point3 = Point { x: -2.5, y: 1.5 };
println!("{:?}", classify_point(point3));
 }

