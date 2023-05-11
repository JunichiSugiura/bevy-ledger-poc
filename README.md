## Ledger Bevy Plugin

Ledger internal hackathon 2023 spring 

### Scope

Create a bevy plugin to interact with ledger devices. Also provide UI for demo.

### Tasks

- [x] Scan Ledger devices
- [x] Open HID transport
- [ ] APDU Commands
  - [x] Get device info
  - [x] Open device app
  - [ ] fix: Ledger device: communication error `response was too short`
  - [ ] fix: Ledger device: Io error
  - [ ] feat: Let user choose a device instead of automatically select the first scanned one

### Usage

```sh
cargo run
```

### Dependencies

- [ruabmbua/hidapi-rs](https://github.com/ruabmbua/hidapi-rs): Rust bindings for the hidapi C library
- [bevyengine/bevy](https://github.com/bevyengine/bevy): A refreshingly simple data-driven game engine built in Rust

### References

- [Zondax/ledger-rs](https://github.com/Zondax/ledger-rs)
- [Ledger APDU spec](https://ledgerhq.atlassian.net/wiki/spaces/WALLETCO/pages/3753377984/An+attempt+at+APDU+specs#openApp-e0d80000xx): Shout out to @juan-cortes !
- [Ethereum application : Common Technical Specifications](https://github.com/LedgerHQ/app-ethereum/blob/develop/doc/ethapp.adoc)

### Lesson learned

- `hidapi-rs` ships v2 now
  - `ledger-rs` hasn't been migrated yet
- Bevy UI 
  - Not intuitive as React-like declarative UI library
  - `dip` needs to migrate to Bevy v0.10 in order to use with this plugin

### Next step

- Decerialize APDU response
- Cover more APDU commands
- Error handling
- Bevy UI -> dip
  - CLI
  - Desktop: but with Virtual DOM + webview
- WASM support for Web client
- BLE?
