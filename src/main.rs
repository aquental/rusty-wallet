use dotenv::from_filename;
use std::env;

fn main() {
    println!("Hello, world!");
    from_filename(".env").ok();

    let result_descriptor = env::var("WALLET_DESCRIPTOR");
    let descriptor = match result_descriptor {
        Ok(descriptor) => descriptor,
        Err(_) => "No descriptor found".to_string(),
    };
    println!("{}", descriptor);
    dbg!(descriptor);
}
