use laminar::{Packet, Socket};
use std::time::Instant;

fn main() {
    const SERVER_ADDR: &'static str = "127.0.0.1:50201";

    let mut socket = Socket::bind("127.0.0.1:50202").unwrap();
    let packet_sender = socket.get_packet_sender();

    let unreliable = Packet::reliable_unordered(
        SERVER_ADDR.parse().unwrap(),
        String::from("Test1").into_bytes(),
    );
    let unreliable1 = Packet::unreliable(
        SERVER_ADDR.parse().unwrap(),
        String::from("Test2").into_bytes(),
    );
    let unreliable2 = Packet::unreliable(
        SERVER_ADDR.parse().unwrap(),
        String::from("Test3").into_bytes(),
    );
    let unreliable3 = Packet::unreliable(
        SERVER_ADDR.parse().unwrap(),
        String::from("Test4").into_bytes(),
    );
    let unreliable4 = Packet::unreliable(
        SERVER_ADDR.parse().unwrap(),
        String::from("Test5").into_bytes(),
    );

    let test = packet_sender.send(unreliable);
    packet_sender.send(unreliable1);
    packet_sender.send(unreliable2);
    packet_sender.send(unreliable3);
    packet_sender.send(unreliable4);
    socket.manual_poll(Instant::now());
}
