use crate::zmq::ZMQWrapper;
use crate::event_loop::AnnaState;
use anna_core::protos::request::{SeedResponse, Server};
use protobuf::Message;

pub trait Handler {
    fn handle(&mut self, socket: &mut ZMQWrapper, state: &mut AnnaState);
}

pub struct SeedHandler;

impl Handler for SeedHandler {
    fn handle(&mut self, socket: &mut ZMQWrapper, _: &mut AnnaState) {
        let mut response = SeedResponse::new();
        for s in global_hash_ring.get_servers() {
            let mut server = Server::new();
            server.set_private_ip(s.private_ip.clone());
            server.set_public_ip(s.public_ip.clone());
            response.servers.push(server);
        }
        socket.send_bytes(response.write_to_bytes().unwrap());
    }
}