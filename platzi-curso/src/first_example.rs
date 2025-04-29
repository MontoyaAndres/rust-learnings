fn main() {
    println!("Please enter your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Please enter your age:");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age: u8 = age.trim().parse().unwrap();

    println!("Hello, {}! You are {} years old.", name.trim(), age);
}
