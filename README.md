# gt7_portforward

Forward GT7 telemetry packets to multiple addresses.

## Usage

```sh
# connect to a PlayStation at 192.168.1.158
# and forward packets to ports 33741 and 33742
cargo run -- --from-ip 192.168.1.158 --to-port 33741 33742

# [INFO ] listening for packets on 0.0.0.0:33740
# [INFO ] connecting to PlayStation on 192.168.1.158:33739
# [INFO ] forwarding to 127.0.0.1:33741
# [INFO ] forwarding to 127.0.0.1:33742
```

```sh
# connect to a PlayStation at 192.168.1.158
# and forward packets to port 33741 (the default)
cargo run -- -f 192.168.1.158

# [INFO ] listening for packets on 0.0.0.0:33740
# [INFO ] connecting to PlayStation on 192.168.1.158:33739
# [INFO ] forwarding to 127.0.0.1:33741
```

```sh
# view a help message
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
