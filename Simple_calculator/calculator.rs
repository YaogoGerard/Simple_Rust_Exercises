use std::io;

fn main(){
loop{
    println!("==== Simple Calculator ===");
    println!("1.Addition");
    println!("2.Soustraction");
    println!("3.Multiplication");
    println!("4.Division");
    println!("0.Leave the calculator")
    println!("Make your choice:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("reading error");

    println!("Enter your first number:");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).expect("reading error");

    println!("Enter your first number:");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).expect("reading error");

}
}
