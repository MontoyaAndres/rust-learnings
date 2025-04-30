fn main() {
    let mut names: Vec<String> = Vec::new();

    for _i in 0..3 {
        println!("Please enter a name:");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        names.push(name.trim().to_string());
    }

    println!("{:?}", names);
}
