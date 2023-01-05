//use crate::archive::arch::archive; // Method 1
use crate::archive::arch::archive as arc; // Method 2
use rand::{Rng, thread_rng}; //using multiple crates example

mod archive;

fn main() {
    println!("Hello, world!");
    //archive("test.txt"); //  Method 1
    arc("test.txt"); // Method 2

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    random_number_examples();

}

fn random_number_examples() {
    let mut rng = thread_rng(); //rand::thread_rng(); works too but not required because I added it to the use statement
    let x: i32 = rng.gen();
    println!("thread_rng is: {}", x); // Random number from -2,147,483,648 to 2,147,483,647 because its a 32 bit integer
    println!("thread_rng is: {}", rng.gen_range(0, 100)); // Range is 1 to 100
    println!("thread_rng is: {}", rng.gen_range(0.0, 100.0)); // Range is 1.0 to 100.0

    let rand_string = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .collect::<String>();
    println!("thread_rng is: {}", rand_string);

}
