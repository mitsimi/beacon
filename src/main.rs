use clap::Parser;
use crossterm::{
    style::Stylize,
    terminal::{Clear, ClearType},
};
use local_ip_address::list_afinet_netifas;
use std::net::UdpSocket;

#[derive(Parser, Debug)]
#[command(name = "wol-listen")]
#[command(about = "Listen for Wake-on-LAN magic packets", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "9")]
    port: u16,

    #[arg(short, long, default_value = "0.0.0.0")]
    interface: String,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn is_magic_packet(data: &[u8]) -> bool {
    if data.len() < 102 {
        return false;
    }

    data[..6].iter().all(|&b| b == 0xFF)
}

fn extract_target_mac(data: &[u8]) -> Option<String> {
    if data.len() < 12 {
        return None;
    }

    let target = &data[6..12];
    Some(format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        target[0], target[1], target[2], target[3], target[4], target[5]
    ))
}

fn print_header(port: u16, interface: &str) {
    println!("{}", Clear(ClearType::All));
    println!(
        "{}\n{}\n{}",
        "╔════════════════════════════════════════════════════════════════╗"
            .bold()
            .green(),
        "║                   Wake-on-LAN Packet Listener                  ║"
            .bold()
            .green(),
        "╚════════════════════════════════════════════════════════════════╝"
            .bold()
            .green()
    );
    println!();

    let bind_addr = format!("{}:{}", interface, port);
    println!("  {}", "Listening on:".bold().cyan());
    println!("    {}", format!("Address: {}", bind_addr).white());
    println!();

    println!("  {}", "Network Interfaces:".bold().cyan());
    if let Ok(interfaces) = list_afinet_netifas() {
        for (name, ip) in &interfaces {
            println!(
                "    {} → {}",
                name.clone().bold().white(),
                ip.to_string().green()
            );
        }
    } else {
        println!("    {}", "Unable to detect interfaces".yellow());
    }

    println!();
    println!(
        "{}",
        "────────────────────────────────────────────────────────────────".dark_grey()
    );
    println!();
    println!(
        "  {}  Waiting for magic packets... (Ctrl+C to stop)",
        "".bold().blue()
    );
    println!();
}

fn print_magic_packet(src: &str, mac: &str, timestamp: &str) {
    println!(
        "{}",
        "╭────────────────────────────────────────────────────────────────╮"
            .bold()
            .green()
    );
    println!(
        "{}",
        "│  ✨  MAGIC PACKET RECEIVED                                     │"
            .bold()
            .green()
    );
    println!(
        "{}",
        "╰────────────────────────────────────────────────────────────────╯"
            .bold()
            .green()
    );
    println!();
    println!("  {}  {}", "Timestamp:".bold().cyan(), timestamp.white());
    println!("  {}  {}", "Source:".bold().cyan(), src.white());
    println!("  {}  {}", "Target MAC:".bold().cyan(), mac.bold().yellow());
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let args = Args::parse();

    print_header(args.port, &args.interface);

    let bind_addr = format!("{}:{}", args.interface, args.port);
    let socket = UdpSocket::bind(&bind_addr)?;

    let mut buf = [0u8; 1024];

    loop {
        let (received, src) = socket.recv_from(&mut buf)?;
        let data = &buf[..received];

        if is_magic_packet(data) {
            let timestamp = jiff::Timestamp::now().strftime("%H:%M:%S%.3f").to_string();
            let mac = extract_target_mac(data).unwrap_or_else(|| "unknown".to_string());

            print_magic_packet(&src.to_string(), &mac, &timestamp);
        } else if args.verbose {
            println!("[DEBUG] Ignored packet from {} ({} bytes)", src, received);
        }
    }
}
