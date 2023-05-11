## Ledger Bevy Plugin

Ledger internal hackathon 2023 spring 

### Scope

Create a bevy plugin to sync eth accounts from ledger devices. Also provide UI for demo.

### Features

- [x] Scan Ledger devices
- [x] Open HID transport
- [ ] APDU Commands
  - [ ] Get device info
    - [ ] Ledger device: communication error `response was too short`
    - [ ] Let user choose a device instead of automatically select the first scanned one
  - [ ] Derive ETH addresses
    - [ ] Ledger device: communication error `response was too short`
    - [ ] Let user choose a device instead of automatically select the first scanned one

### Usage

```sh
cargo run
```

### Dependencies

- [ruabmbua/hidapi-rs](https://github.com/ruabmbua/hidapi-rs): Rust bindings for the hidapi C library
- [bevyengine/bevy](https://github.com/bevyengine/bevy): A refreshingly simple data-driven game engine built in Rust

### References

- [Zondax/ledger-rs](https://github.com/Zondax/ledger-rs): Shout out to @juan-cortes !
- [Ledger APDU spec](https://ledgerhq.atlassian.net/wiki/spaces/WALLETCO/pages/3753377984/An+attempt+at+APDU+specs#openApp-e0d80000xx)
- [Ethereum application : Common Technical Specifications](https://github.com/LedgerHQ/app-ethereum/blob/develop/doc/ethapp.adoc)
