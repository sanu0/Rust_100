/**
 * We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: 
 * The program will generate a random integer between 1 and 100. It will then prompt the player to 
 * enter a guess. After a guess is entered, the program will indicate whether the 
 * guess is too low or too high. If the guess is correct, the game will print a 
 * congratulatory message and exit.
 */

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();   //String is a type and new() is associated function of the String type

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //The read_line returns a enum Result, it can have to variants Ok and Err
    //If it is Err then expect method defined on Result type will cause the program to stop and 
    //display the text it takes as an argument and if it is Ok then expect function will just show that value.

    println!("You guessed  {guess}");

    



}
