use std::io;

fn main() {
    let mut list: Vec<String> = Vec::new();
    loop {
        println!("1. Add an article");
        println!("2. show the complete list");
        println!("3. Remove an article");
        println!("0. Leave Rmanager");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Reading Error");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };
        if choice == 0 {
            println!("Thank you, Good bye!");
            break;
        }
        match choice {
            1 => {
                println!("Enter the name of your article:");
                let mut article = String::new();
                io::stdin().read_line(&mut article).expect("Readind Error");
                if list.contains(&article.trim().to_string().to_lowercase()) {
                    println!("This article already exist")
                } else {
                    list.push(article.trim().to_string());
                    println!("{} added succesfully", article.trim());
                }
            }
            2 => {
                println!("{:-^25}", "The Articles List");
                for (index, article) in (&list).iter().enumerate() {
                    if list.is_empty() {
                        println!("Article list is empty");
                    } else {
                        println!("{}.{}", index + 1, article);
                    }
                }
            }
            3 => {
                if list.is_empty() {
                    println!("La liste est vide, rien à supprimer.");
                    continue;
                }
                println!("{:-^25}", "The Articles List");
                for (index, article) in (&list).iter().enumerate() {
                    if list.is_empty() {
                        println!("Article list is empty");
                    } else {
                        println!("{}.{}", index + 1, article);
                    }
                }
                println!("Choice the number of article to remove:");
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Reading Eror");
                let choice: i32 = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid choice");
                        continue;
                    }
                };
                for index in 0..list.len() {
                    if index == (choice - 1) as usize {
                        list.remove(index);
                        println!("the article is removed");
                    }
                }
            }

            _ => println!("Invalid choice"),
        }
    }
}
