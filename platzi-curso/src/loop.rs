fn main() {
    let number_one = 123;
    let number_two = 321;

    loop {
        println!("Please enter the sum of {} and {}", number_one, number_two);

        let mut sum = String::new();
        std::io::stdin().read_line(&mut sum).unwrap();

        let sum: i16 = sum.trim().parse().unwrap();

        if sum == number_one + number_two {
            println!("Correct!");
            break;
        } else {
            println!("Incorrect! Please try again.");
        }
    }
}
