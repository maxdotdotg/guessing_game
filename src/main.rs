// use the standard io lib
use std::io;

fn main() {
    println!("Guess the number!");
    println!("please input your guess.");

    // creat the guess variable
    // it's a new string (not sure wtf that means)
    // create guess
    // it's type is string, and it's new (associated type wtf)
    // and it's new so it's empty?
    // a new empty instace of a string?
    //
    // and it's mutable! it can change?
    // I think it's because we don't know what size guess will be
    // before runtime. it could be really big, as in many bytes
    // or characters, or it could be small
    let mut guess = String::new();


    // open stdin and read upto the newline
    // store all that jazz in the guess variable
    // updating it
    // "The string argument needs to be mutable so the method can change the string’s content by
    // adding the user input."
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);

}
