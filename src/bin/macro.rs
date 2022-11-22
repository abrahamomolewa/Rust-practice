

macro_rules! add {
    ($($item:expr),*) => {
        println!("The sum of this is {}", 0 $(+$item)*);
    }
}

fn main() {

    add!(2,3);
}