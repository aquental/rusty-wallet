use bdk::{bitcoin::Network, database::MemoryDatabase, wallet::Wallet};
use dotenv;
use dotenv::from_filename;

fn main() -> anyhow::Result<()> {
    println!("Rusty Bitcoin Wallet.");
    from_filename(".env").ok();
    dotenv::dotenv().ok();

    let result_descriptor = std::env::var("WALLET_DESCRIPTOR");
    let descriptor = match result_descriptor {
        Ok(descriptor) => descriptor,
        Err(_) => "No descriptor found".to_string(),
    };
    println!("{}", descriptor);
    //dbg!(descriptor);

    let wallet = Wallet::new(
        &descriptor.clone(),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
    println!("{:?}", wallet);

    Ok(())
}
