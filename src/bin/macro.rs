//Question1: Implement a macro named addition to add any amount of numbers. 

macro_rules! add {
    ($($item:expr),*) => {
        println!("The sum of this is {}", 0 $(+$item)*);
    }
}

fn main() {

    add!(2,3,5,6,7,7);
}