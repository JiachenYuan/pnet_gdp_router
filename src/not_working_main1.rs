extern crate pnet_macros_support;
mod packet;

use pnet::packet::ip::IpNextHeaderProtocols;
// use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::TransportChannelType::Layer3;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{transport_channel, ipv4_packet_iter, udp_packet_iter};


// use packet::my_protocol::{MyProtocolPacket, MutableMyProtocolPacket};
// use pnet_packet::PacketSize;
// use pnet_packet::udp::UdpPacket;
use packet::GDP_protocol::{GdpProtocolPacket, MutableGdpProtocolPacket};


fn handle_gdp_packet(packet: &[u8]) {
    let gdp_protocol_packet = GdpProtocolPacket::new(packet);
    if let Some(gdp) = gdp_protocol_packet {
        println!("{:?}", gdp);
    } else {
        println!("Malformed GDP Packet");
    }
}



fn main() {
    let protocol = Layer3(IpNextHeaderProtocols::Udp);

    // Create a new transport channel, dealing with layer 4 packets on a test protocol
    // It has a receive buffer of 4096 bytes.
    let (mut tx, mut rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!(
            "An error occurred when creating the transport channel: {}",
            e
        ),
    };

    // We treat received packets as if they were ip packets
    let mut iter = udp_packet_iter(&mut rx);
    loop {
        match iter.next() {
            Ok((packet, addr)) => {
                // Allocate enough space for a new packet
                // let mut vec: Vec<u8> = vec![0; packet.packet().len()];
                // let mut new_packet = MutableIpv4Packet::new(&mut vec[..]).unwrap();

                // println!("{:?}", packet.packet().len());
                // println!("{:?}", packet.packet_size());
                // // Create a clone of the original packet
                // new_packet.clone_from(&packet);

                // Switch the source and destination ports
                // new_packet.set_source(packet.get_destination());
                // new_packet.set_destination(packet.get_source());

                // Send the packet
                // match tx.send_to(new_packet, addr) {
                //     Ok(n) => assert_eq!(n, packet.packet().len()),
                //     Err(e) => panic!("failed to send packet: {}", e),
                // }

                println!("{:?}", packet);


                let udp_payload = packet.payload();
                // Assume all udp packet sent to 31415 is a valid GDP packet
                // todo: validate Gdp packet
                if packet.get_destination() == 31415 {
                    handle_gdp_packet(udp_payload);
                }
                


            }
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}