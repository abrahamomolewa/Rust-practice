use std::io;


//question 1: Create an enum that contains HTTP 4xx Client Errors i.e. Not Found Should have value 404 associated with it.
enum ClientErrors {
    BadRequest,
    Unauthorized,
    Forbidden,
    PageNotFound 
 }
 
 fn matchErrors(error: ClientErrors) -> u32 {
     match error { 
      ClientErrors::BadRequest => 400,
      ClientErrors::Unauthorized => 401,
      ClientErrors::Forbidden => 403,
      ClientErrors::PageNotFound => 404
     }
 }
 
 fn printerror() {
       
     println!("{}", matchErrors(ClientErrors::BadRequest));
     println!("{}", matchErrors(ClientErrors::Unauthorized));
     println!("{}", matchErrors(ClientErrors::Forbidden));
     println!("{}", matchErrors(ClientErrors::PageNotFound));
 
 }
 //////
 
 //Question 2: Write an enum to store information of whether a person is a child or adult based on his/her age.
 enum Person {
    AdultOrNot(i32),

}
///
 
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

 }

