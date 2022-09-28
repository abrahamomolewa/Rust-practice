use std::io;



fn main() {
    //Write a program to print Hello, World.
    println!("Hello World");

/*Write a program to print a block F using hash (#), where the F has a height of six
characters and width of five and four characters. */
println!("######");
println!("#");
println!("#");
println!("#####");
println!("#");
println!("#");
println!("#");

//Create a tuple to hold an i32 and f32 and then call the f32 value.
let result: (i32, f32) = (5, 7.7);

println!("{}", result.1);

//Take input(name) from the user of type String and print Hello, <name>!.
println!("Type your text here");
let mut input_text = String::new();
io::stdin().read_line(&mut input_text).expect("NAN");
println!("hello {}", input_text);

//Initialize an array of sixteen 0
let array = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

println!("{}", array.len());

}
