use std::thread;
use std::time::Duration;

//Question 1: Find out the number of physical and logical cores in your CPU using rust. hint: try using num_cpus crate.
fn main()  { 

let cpus = num_cpus::get();
if cpus > 1 {
    println!("you are on a multicore system with {} CPUs", cpus);
} else {
    println!("you are on a single core system");
};
let logical_cpus = num_cpus::get();
let physical_cpus = num_cpus::get_physical();
if logical_cpus > physical_cpus {
    println!("We have simultaneous multithreading with about {:.2} \
              logical cores to 1 physical core.", 
              (logical_cpus as f64) / (physical_cpus as f64));
} else if logical_cpus == physical_cpus {
    println!("Either we don't have simultaneous multithreading, or our \
              system doesn't support getting the number of physical CPUs.");
} else {
    println!("We have less logical CPUs than physical CPUs, maybe we only have access to \
              some of the CPUs on our system.");
};
////

//question 2: Spawn 2 threads, one that continuously says Hello and the other that say World.

let handle = thread::spawn(|| {
    for i in 1..2 {
        println!("hello {}", i );
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();

for i in 1..2 {
    println!("world! {}", i);
    thread::sleep(Duration::from_millis(1));
}
}