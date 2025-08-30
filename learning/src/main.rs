use learning::{Key, get_all_features, init_features, read_key};

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_features();

    let features: Vec<(&'static str, fn())> = get_all_features().into_iter().collect();

    loop {
        for (index, (name, _)) in features.iter().enumerate() {
            println!("{:2}. {}", index + 1, name);
        }

        println!(
            "\nEnter the number of the feature to run (1-{}), or any other key to exit:",
            features.len()
        );

        let input: Key = read_key()?;
        match input {
            Key::Esc | Key::Invalid => break,
            Key::Digit(n) => {
                let choice: usize = n as usize;
                if choice >= 1 && choice <= features.len() {
                    if let Some((_, func)) = features.get(choice - 1) {
                        println!("Running {}...", features[choice - 1].0);
                        func();
                        println!("Finished {}...", features[choice - 1].0);
                        println!("Press Enter to continue...");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}
