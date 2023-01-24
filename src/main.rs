use std::io;
// getting input/output library into scope from Std (standard library)
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut x = 5;
    println!("value of x is {x}");
    x = 10;
    println!("value of x is {x}");

    let y = 10;
    let y = y * 10;

    {
        let y = y * 20;
        println!("y inner scope value is {y}");
    }
    println!("y's outer scope value is {y}");

    let spaces = "nikhil";
    let length = spaces.len();
    println!("length of spaces is {length}");

    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!(" The secret number is {secret_number}");
    // loop {
    println!("Please input your guess.");

    //  let to create a variable example: let name = "nikhil" this var is imutable by default
    // if I put mut after let then it will be mutable

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Please type a Number!");

    println!("you Guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too chota"),
        Ordering::Greater => println!("Bohut Badda"),
        Ordering::Equal => println!("Jiit Gye Bhaii"),
    }
    // }
}
