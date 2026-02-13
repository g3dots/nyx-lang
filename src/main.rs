use nyx::repl;

fn main() {
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "stranger".to_string());

    println!("Nyx 0.1.0");
    println!("Hello, {username}. Type an expression to get started.");

    repl::start();
}
