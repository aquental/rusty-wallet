use std::env;

use dotenv::dotenv;

fn main() {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;
}
