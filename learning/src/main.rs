use learning::{get_all_features, init_features};
use std::io;
use std::io::Write;

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    const EXIT_SIGNAL: usize = 0;

    init_features()?;

    let mut features: Vec<(&'static str, fn())> = get_all_features()?.into_iter().collect();
    features.sort_by_key(|&(key, _)| key);

    let mut input_buffer: String = String::new();
    loop {
        clear_screen();

        for (index, (name, _)) in features.iter().enumerate() {
            println!("{:4}. {}", index + 1, name);
        }

        println!(
            "Enter the number of the feature to run (1-{}), or 0 to exit:",
            features.len()
        );

        // show messages immediately
        io::stdout().flush()?;

        // read user input
        input_buffer.clear();
        let read_result = io::stdin().read_line(&mut input_buffer);

        if let Err(e) = read_result {
            eprintln!("Failed to read user input: {}", e);
            continue;
        }

        // parse input number
        let input_num = match input_buffer.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Please type number");
                println!("Press any key to continue...");
                io::stdin().read_line(&mut String::new())?;
                continue;
            }
        };

        match input_num {
            EXIT_SIGNAL => {
                println!("Exiting...");
                break;
            }

            num if num >= 1 && num <= features.len() => {
                if let Some((_, func)) = features.get(num - 1) {
                    clear_screen();
                    println!("{}...Begin", features[num - 1].0);
                    func();
                    println!("{}...End", features[num - 1].0);
                    println!("Press any key to continue...");
                    io::stdin().read_line(&mut String::new())?;
                }
            }

            _ => {
                eprintln!(
                    "Invalid input，please input number between 0 to {}",
                    features.len()
                );
                println!("Press any key to continue...");
                io::stdin().read_line(&mut String::new())?;
                continue;
            }
        }
    }

    Ok(())
}

fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("sh")
            .args(&["-c", "clear"])
            .status();
    }
}
