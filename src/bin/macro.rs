macro_rules! add {
    ($item:expr, $item2:expr) => {
        println!("The sum of this is {}", $item + $item2);
    }
}

fn main() {

    add!(2,3);
}