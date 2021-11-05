use crate::event_loop::{AnnaEventLoop, AnnaState};
use crate::hash_ring::HashRing;
use crate::zmq::{ZMQSocketCache, ZMQWrapper};
use crate::server_thread::ServerThread;
use anna_core::protos::request::{SeedResponse, Server};
use protobuf::Message;
use crate::kv_store::KvStore;
use crate::handlers::SeedHandler;
use zmq::{Context, PollEvents};

pub struct InitHandler {
    pub server_thread: ServerThread,
    pub thread_count: usize,
    pub seed_ip: String,
    pub is_seed_node: bool
}

impl InitHandler {
    pub fn init(&self) -> AnnaEventLoop {
        let zmq_context = zmq::Context::new();
        let mut global_hash_ring: HashRing = HashRing::new_global();
        let mut local_hash_ring: HashRing = HashRing::new_local();

        let mut socket_pool = ZMQSocketCache::new();
        self.init_global_hash_ring(&mut global_hash_ring);
        self.init_local_hash_ring(&mut local_hash_ring);

        // Thread 0 notifies other nodes
        if self.server_thread.thread_id == 0 {
            self.notify_other_servers(&global_hash_ring, &mut socket_pool);
        }

        let mut event_loop = AnnaEventLoop::new(
            AnnaState {
                kv_store: KvStore::new(),
                global_hash_ring,
                local_hash_ring,
                socket_pool,
                server_thread
            }
        );

        // Register sockets and handlers
        self.register_seed_node(&zmq_context, &mut event_loop);

        return event_loop;
    }

    fn register_seed_node(&self, zmq_context: &Context, event_loop: &mut AnnaEventLoop) {
        let seed_socket = ZMQWrapper {
            socket: zmq_context.socket(zmq::REP).unwrap()
        };
        seed_socket.bind(self.get_seed_addr().as_str());
        event_loop.register(
            seed_socket,
            seed_socket.as_poll_item(PollEvents::POLLIN),
            SeedHandler{})
    }

    pub fn notify_other_servers(&self, global_hash_ring: &HashRing, socket_pool: &mut ZMQSocketCache) {
        let mut server = Server::new();
        server.set_public_ip(self.server_thread.public_ip.clone());
        server.set_private_ip(self.server_thread.private_ip.clone());

        for s in global_hash_ring.get_servers() {
            if s.private_ip == self.private_ip {
                continue;
            }
            let socket = socket_pool.get_or_connect(self.get_seed_connect_addr(s.private_ip.clone()), zmq::PUSH);
            socket.send_bytes(server.write_to_bytes().unwrap());
        }
    }

    pub fn init_local_hash_ring(&self, local_hash_ring: &mut HashRing) {
        for i in 0..self.thread_count {
            local_hash_ring.insert(ServerThread {
                public_ip: self.public_ip.clone(),
                private_ip: self.private_ip.clone(),
                thread_id: i,
                virtual_num: 0,
            }, 0);
        }
    }

    pub fn init_global_hash_ring(&self, global_hash_ring: &mut HashRing) {
        let seed_req_socket = socket_cache.get_or_connect(self.server_thread.get_seed_connect_addr(seed_ip), zmq::REQ);
        seed_req_socket.send_string("join me");
        let data = seed_req_socket.recv_bytes();
        if data.is_some() {
            let seed_response = SeedResponse::parse_from_bytes(&data.unwrap()).unwrap();
            for s in seed_response.servers {
                global_hash_ring.insert(ServerThread {
                    public_ip: s.public_ip,
                    private_ip: s.private_ip,
                    thread_id: 0,
                    virtual_num: 0,
                }, 0);
            }
        }

        // Todo - get join count for this new server
        global_hash_ring.insert(ServerThread {
            public_ip: self.public_ip.clone(),
            private_ip: self.private_ip.clone(),
            thread_id: 0,
            virtual_num: 0,
        }, 0);
    }
}