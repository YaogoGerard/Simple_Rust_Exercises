/*
 Write a 'Hello World' program that declares variables,
 prompts the user to enter their name, and greets them back, 
 while learning the concepts of mutability, ownership, 
 and the difference between String and &str
 */



use std::io;

fn main (){
    println!("Hello, Could you give me your name:");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("reading error");
    println!("Bonjour {}",name.trim());
}
