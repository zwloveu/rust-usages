pub fn process_number_without_guard(number: i32) {
    match number {
        0 => println!("0"),
        x => {
            if x > 0 && x < 5 {
                println!("1-4");
            } else if x > 5 {
                println!(">5");
            } else {
                println!("<0")
            }
        }
    }
}
