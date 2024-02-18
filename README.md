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
