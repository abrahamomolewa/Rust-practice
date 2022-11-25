

//Quesetion 3:  Create a function that accepts a closure
#[derive(Debug)]
struct Book {
    name: String,
    year: u32
}

impl Book { 
fn change<F>(&mut self, f: F)
where F: Fn(&mut Book) {
    f(self);
}
}
///

fn main() {
//question 1: Create a closer add_one to add 1 to an integer.

let add_one = |x: u32| {
    println!("{}", x + 1);
};

add_one(6+54);
//end of question 1

 //Question 2: Create an vec of numbers from 0 ... 100, mutate the vec so that it does not contain any number divisible by 3
    let vals: Vec<u32> = (0..100).filter_map(|v| if v % 3 != 0 {
        Some(v)
    } else {None}).collect();

    println!("{:?}", vals);
//end of quesetion 2




//Question 3: body

let mut book = Book {
    name: "The best book".to_string(),
    year: 2000
};

book.change(|cool_book| {
    cool_book.name = "the worst book".to_string();

   if  cool_book.year % 2 == 0 {
        println!("the number is even");
    } else {
        println!("the book is odd");
    }
    cool_book.year = 7000
});

println!("{:?}", book);

//end of question 3;

}