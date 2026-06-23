use std::io;

fn main() {
    loop {
        println!("==== Simple Calculator ===");
        println!("1.Addition");
        println!("2.Soustraction");
        println!("3.Multiplication");
        println!("4.Division");
        println!("0.Leave the calculator");
        println!("Make your choice:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("reading error");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        if choice == 0 {
            println!("Good bye !");
            break;
        }

        println!("Enter your first number:");
        let num1 = read_number();

        println!("Enter your second number:");
        let num2 = read_number();

        match choice {
            1 => println!("{} + {} = {}", num1, num2, num1 + num2),
            2 => println!("{} - {} = {}", num1, num2, num1 - num2),
            3 => println!("{} * {} = {}", num1, num2, num1 * num2),
            4 => {
                if num2 == 0 {
                    println!("Zero division error !");
                } else {
                    println!("{} / {} = {}", num1, num2, num1 as f64 / num2 as f64);
                }
            }
            _ => println!("Invalid choice !"),
        }
    }
}

fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Reading error");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
