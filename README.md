# rusty-wallet

Simple Bitcoin Wallet implementation in Rust.
Uses [BitcoinDevKit/DBK](https://github.com/bitcoindevkit/bdk)

# Step Zero

```script
rustup
rust analyzer
```

# Step One

Make an app the imports bdk, reads xpub from the environment variable, outputs first unused address and quits.
Read descriptor from env.

```script
# read from file
cargo add dotenv
# error handling
cargo add anyhow
# add Bitcoin Dev Kit
cargo add bdk
```

https://youtu.be/md-ecvXBGzI?t=3179
