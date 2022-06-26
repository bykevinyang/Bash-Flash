use std::collections::HashMap;

mod bash_flash;
mod card;

fn main() {
    bash_flash::start::run();
    let mut set = set::Set {
        title: "Les Mots De Liason".to_string(),
        subject: "French".to_string(),
        cards: HashMap::new(),
    };
    println!("{:?}", set);
    set.add_card("Joe".to_string(), "Mama".to_string());
    println!("{:?}", set);
    set.remove_card("Joe".to_string());
    println!("{:?}", set);

    // // Reading from console:
    // let mut line = String::new();
    // println!("Enter your name : ");
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("Hello, {}", line);
    // println!("no of bytes read , {}", b1);

    // // Writing to console:
    // let b1 = std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
    // let b2 = std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
    // std::io::stdout().write(format!("\nbytes written {}", (b1+b2)).as_bytes()).unwrap();
}