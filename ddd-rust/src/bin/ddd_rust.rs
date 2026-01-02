use ddd_rust::bootstrap::run_ddd_rust;

fn main() {
    if let Err(err) = run_ddd_rust() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
