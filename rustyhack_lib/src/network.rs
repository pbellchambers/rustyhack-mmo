pub mod packets;

use crossbeam_channel::Sender;
use laminar::Packet;

pub fn send_packet(packet: Packet, sender: &Sender<Packet>) {
    let send_result = sender.send(packet);

    #[allow(clippy::ignored_unit_patterns)]
    match send_result {
        Ok(_) => {
            //packet send successful
        }
        Err(message) => {
            warn!("Error sending packet: {}", message);
            warn!("Will try to continue, but things may be broken.");
        }
    }
}
