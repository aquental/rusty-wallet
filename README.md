# rusty-wallet

Simple Bitcoin Wallet implementation in Rust.
Uses [BitcoinDevKit/DBK](https://github.com/bitcoindevkit/bdk)

Based on [Learn Rust (by building a simple bitcoin wallet) with Paul](https://www.youtube.com/watch?v=md-ecvXBGzI) by [PlebLab](https://www.youtube.com/@pbs_plebs) website: [pleblab.dev](https://www.pleblab.dev/).

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

stopped at: https://youtu.be/md-ecvXBGzI?t=3179
