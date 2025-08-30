use learning::{get_all_features, init_features};
use std::io;

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    const EXIT_SIGNAL: usize = 0;

    init_features();

    let mut features: Vec<(&'static str, fn())> = get_all_features().into_iter().collect();
    features.sort_by_key(|&(key, _)| key);

    let mut input_buffer: String = String::new();
    loop {
        for (index, (name, _)) in features.iter().enumerate() {
            println!("{:2}. {}", index + 1, name);
        }

        println!(
            "Enter the number of the feature to run (1-{}), or 0 to exit:",
            features.len()
        );

        input_buffer.clear();
        let read_bytes = io::stdin().read_line(&mut input_buffer)?;

        let input_num = match input_buffer.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                clear_screen();
                continue;                
            }
        };

        match (read_bytes, input_num) {
            (2, EXIT_SIGNAL) => {
                break;
            }

            (2, num) if (1..=features.len()).contains(&num) => {
                if let Some((_, func)) = features.get(num - 1) {
                    println!("Running {}...", features[num - 1].0);
                    func();
                    println!("Finished {}...", features[num - 1].0);
                    println!("Press any key to continue...");
                    let mut input: String = String::new();
                    io::stdin().read_line(&mut input)?;                    
                }                
            }

            _ => {
                continue;
            }
        }
        clear_screen();
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
