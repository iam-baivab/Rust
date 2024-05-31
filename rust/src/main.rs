use std::io;
fn main()
{
    print!("Enter your name: ");
    let mut input = String::new();

    // standard input function from io module -> stdin() method
    // Passing a mutable reference to a where we want to store the input -> read_line() method
    // expect() method to handle any errors -> expect() method
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Hello, {}", input);
}