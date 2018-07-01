#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate pnet;
extern crate serde;
extern crate toml;

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::packet::arp;
use pnet::packet::{tcp, udp};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use std::fs;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
struct Config {
    interface: Option<InterfaceConfig>,
}

impl Config {
    fn extract_interface_name(&self) -> Option<String> {
        self.interface.as_ref()?.name.as_ref().cloned()
    }
}

#[derive(Debug, Deserialize)]
struct InterfaceConfig {
    name: Option<String>,
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

fn handle_packet(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Arp => {
            let arp = arp::ArpPacket::new(ethernet.payload()).unwrap();
            match arp.get_operation() {
                arp::ArpOperations::Reply => {
                    println!(
                        "ARP reply({}): {} -> {}",
                        arp.get_sender_proto_addr(),
                        arp.get_sender_hw_addr(),
                        arp.get_target_hw_addr()
                    );
                }
                arp::ArpOperations::Request => {
                    println!(
                        "ARP request({}): {} -> {}",
                        arp.get_target_proto_addr(),
                        arp.get_sender_hw_addr(),
                        arp.get_target_hw_addr()
                    );
                }
                _ => (),
            }
        }
        EtherTypes::Ipv4 => {
            let ip = Ipv4Packet::new(ethernet.payload()).unwrap();
            //            println!("{} -> {}", ip.get_source(), ip.get_destination());
            handle_l4_packet(&interface, &ip);
        }
        _ => (),
    }
}

fn handle_l4_packet(_interface: &NetworkInterface, ip: &Ipv4Packet) {
    match ip.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            let _tcp = tcp::TcpPacket::new(ip.payload()).unwrap();
            //            println!("{} -> {}", tcp.get_source(), tcp.get_destination());
        }
        IpNextHeaderProtocols::Udp => {
            let _udp = udp::UdpPacket::new(ip.payload()).unwrap();
            //            println!("{} -> {}", udp.get_source(), udp.get_destination());
        }
        _ => (),
    }
}

fn main() -> Result<(), String> {
    let s = match read_file("./Router.toml".to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read config file: {}", e),
    };
    let config: Config = toml::from_str(&s).map_err(|e| e.to_string())?;

    let interface_name = config
        .extract_interface_name()
        .ok_or("extract name failed.")?;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name == interface_name)
        .ok_or(format!("interface_name={} was not found.", interface_name))?;

    let (mut _tx, mut rx) = datalink::channel(&interface, Default::default())
        .map(|chan| match chan {
            Ethernet(tx, rx) => (tx, rx),
            _ => panic!("Unhandled channel type"),
        })
        .map_err(|e| {
            format!(
                "An error occurred when creating the datalink channel: {}",
                e.to_string()
            )
        })?;

    loop {
        let next_packet = rx.next()
            .map_err(|e| format!("An error occurred when read next packet: {}", e.to_string()))
            .and_then(|packet| {
                EthernetPacket::new(packet).ok_or("failed to parse ethernet packet".to_string())
            });

        match next_packet {
            Ok(packet) => {
                // println!("{}: {} -> {}", packet.get_ethertype(), packet.get_source(), packet.get_destination());
                handle_packet(&interface, &packet);
            }
            Err(err) => {
                error!("failed to read next packet {}, ignore and continue.", err);
                continue;
            }
        }
    }
}
