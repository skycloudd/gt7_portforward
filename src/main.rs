use clap::Parser;
use gran_turismo_query::{
    constants::{PACKET_HEARTBEAT_DATA, PACKET_SIZE},
    packet::Packet,
};
use log::{LevelFilter, debug, info};
use std::net::{IpAddr, SocketAddr};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// IP address of the PlayStation console
    #[arg(short, long, value_name = "IP")]
    from_ip: IpAddr,

    /// Game telemetry UDP port
    #[arg(long, value_name = "PORT", default_value_t = 33740)]
    from_port: u16,

    /// IP address to forward packets to
    #[arg(
        short,
        long,
        value_name = "IP",
        default_value_t = IpAddr::from(std::net::Ipv4Addr::new(127, 0, 0, 1))
    )]
    to_ip: IpAddr,

    /// List of ports to forward packets to
    #[arg(
        short = 'p',
        long = "to-port",
        value_name = "PORT",
        num_args = 1..,
        default_value = "33741"
    )]
    to_ports: Vec<u16>,
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .init();

    let args = Args::parse();

    let socket = std::net::UdpSocket::bind(("0.0.0.0", 33740)).unwrap();
    info!("listening for packets on {}", socket.local_addr().unwrap());

    let destination = SocketAddr::new(args.from_ip, 33739);

    info!("connecting to PlayStation on {destination}");

    socket.send_to(PACKET_HEARTBEAT_DATA, destination).unwrap();

    for port in &args.to_ports {
        info!("forwarding to {}", SocketAddr::new(args.to_ip, *port));
    }

    loop {
        socket.send_to(PACKET_HEARTBEAT_DATA, destination).unwrap();

        let mut buf = [0u8; PACKET_SIZE];

        socket.recv_from(&mut buf).unwrap();

        let packet = Packet::try_from(&buf).unwrap();

        if packet.packet_id % 100 == 0 {
            socket.send_to(PACKET_HEARTBEAT_DATA, destination).unwrap();
            debug!("sent heartbeat to {}", destination);
        }

        for port in &args.to_ports {
            let addr = SocketAddr::new(args.to_ip, *port);

            socket.send_to(&buf, addr).unwrap();
            debug!("forwarded packet id {} to {}", packet.packet_id, addr);
        }
    }
}
