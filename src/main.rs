extern crate pnet;

use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::packet::{tcp, udp};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use std::env;

fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
//                println!("{}: {} -> {}", packet.get_ethertype(), packet.get_source(), packet.get_destination());
                handle_packet(&interface, &packet);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
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
            let tcp = tcp::TcpPacket::new(ip.payload()).unwrap();
//            println!("{} -> {}", tcp.get_source(), tcp.get_destination());
        }
        IpNextHeaderProtocols::Udp => {
            let udp = udp::UdpPacket::new(ip.payload()).unwrap();
            println!("{} -> {}", udp.get_source(), udp.get_destination());
        }
        _ => (),
    }
}