# rust-network

## Ethernet frameの表示

### Source

```rust
extern crate pnet;

use pnet::packet::ethernet::EthernetPacket;
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
                println!("{}: {} -> {}", packet.get_ethertype(), packet.get_source(), packet.get_destination());

            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
```

### 実行

MACアドレスだけ実行結果から編集してます。実行すると自分の環境のMACアドレスが表示されます。

```sh
❯ cargo run --package rust-network --bin rust-network en0
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/rust-network en0`
Ipv6: aa:aa:aa:aa:aa:aa -> bb:bb:bb:bb:bb:bb
Ipv6: aa:aa:aa:aa:aa:aa -> bb:bb:bb:bb:bb:bb
Ipv6: bb:bb:bb:bb:bb:bb -> aa:aa:aa:aa:aa:aa
Ipv4: bb:bb:bb:bb:bb:bb -> aa:aa:aa:aa:aa:aa
Ipv4: aa:aa:aa:aa:aa:aa -> bb:bb:bb:bb:bb:bb
```


## IPアドレスの表示

### Source


```rust
extern crate pnet;

use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
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
                handle_packet(&interface, &packet);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(_interface: &NetworkInterface, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ip = Ipv4Packet::new(ethernet.payload()).unwrap();
            println!("{} -> {}", ip.get_source(), ip.get_destination());
        }
        _ => (),
    }
}
```

### 実行

```sh
❯ cargo run --package rust-network --bin rust-network en0
    Finished dev [unoptimized + debuginfo] target(s) in 0.95 secs
     Running `target/debug/rust-network en0`
  192.168.100.101 -> 224.0.0.251
  192.168.100.103 -> 224.0.0.251
```


## references
* [libpnet](https://github.com/libpnet/libpnet)
* [Kyoto.なんか #2 で Rust の実践的な話について発表してきました](http://kizkoh.hatenablog.com/entry/2016/08/26/163216)