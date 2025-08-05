# gt7_portforward

A simple Rust program to forward GT7 telemetry packets to (multiple) ports.

> [!IMPORTANT]
> You need to have Rust installed to run this software. This can (and should) be
> done with [rustup](https://rustup.rs/).

## Usage

### Connect to a PlayStation at `192.168.1.158` and forward packets to ports `33741` and `33742`

```sh
cargo run -- --from-ip 192.168.1.158 --to-port 33741 33742

# [INFO ] listening for packets on 0.0.0.0:33740
# [INFO ] connecting to PlayStation on 192.168.1.158:33739
# [INFO ] forwarding to 127.0.0.1:33741
# [INFO ] forwarding to 127.0.0.1:33742
```

### View a help message

```sh
cargo run -- --help

# Usage: gt7_portforward [OPTIONS] --from-ip <IP>

# Options:
#   -f, --from-ip <IP>       IP address of the PlayStation console
#       --from-port <PORT>   Game telemetry UDP port [default: 33740]
#   -t, --to-ip <IP>         IP address to forward packets to [default: 127.0.0.1]
#   -p, --to-port <PORT>...  List of ports to forward packets to [default: 33741]
#   -h, --help               Print help
#   -V, --version            Print version
```

### Install the program for easier access. You need to have `$HOME/.cargo/bin` in your PATH for this to work.

```sh
cargo install --path .

#   Installing gt7_portforward v0.1.0 (/Users/kyra/rust/gt7_portforward)
#     Updating crates.io index
#      Locking 56 packages to latest Rust 1.87.0 compatible versions
#    Compiling proc-macro2 v1.0.95
#    Compiling version_check v0.9.5
#    ...
#    Compiling gt7_portforward v0.1.0 (/Users/kyra/rust/gt7_portforward)
#     Finished `release` profile [optimized] target(s) in 10.26s
#   Installing /Users/kyra/.cargo/bin/gt7_portforward
#    Installed package `gt7_portforward v0.1.0 (/Users/kyra/rust/gt7_portforward)` (executable `gt7_portforward`)

# You can now run the program whenever you want :3
gt7_portforward --help
```
