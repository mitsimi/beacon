# Beacon

`beacon` is a terminal tool for listening for Wake-on-LAN (WoL) magic packets. It shows the local network interfaces at startup, then prints the time, sender address, and target MAC address for each received magic packet.

## Run

```sh
cargo run -- --port 9 --interface 0.0.0.0
```

The defaults listen on all interfaces (`0.0.0.0`) using UDP port `9`, so the equivalent short command is:

```sh
cargo run
```

Use `--verbose` to log non-WoL UDP packets, or `--help` to see all command-line options.

## Build

```sh
cargo build --release
```

The optimized binary is written to `target/release/beacon`.

## License

MIT
