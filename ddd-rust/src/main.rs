use ddd_rust::infrastructure;

fn main() {
    println!("Hello, Rust");

    let result = infrastructure::dummy_block_on(async {
        std::thread::sleep(std::time::Duration::from_millis(3000));
        42
    });
    println!("Result: {}", result);
}
