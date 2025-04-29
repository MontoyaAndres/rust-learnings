fn main() {
    println!("Please enter your age:");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age: u32 = age.trim().parse().unwrap();

    if age >= 18 {
        println!("You can enter the club.");

        if age >= 20 && age < 30 {
            println!("You are in your twenties.");
        } else if age >= 30 && age < 40 {
            println!("You are in your thirties.");
        } else if age >= 40 && age < 50 {
            println!("You are in your forties.");
        } else if age >= 50 && age < 60 {
            println!("You are in your fifties.");
        } else if age >= 60 {
            println!("You are a senior citizen.");
        }
    } else {
        println!("You cannot enter the club.");
    }
}
