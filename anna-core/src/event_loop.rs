use crate::kv_store::KvStore;
use crate::hash_ring::HashRing;
use crate::zmq::{ZMQSocketCache, ZMQWrapper};
use zmq::{PollItem, PollEvents};
use crate::handlers::Handler;
use crate::server_thread::ServerThread;

pub struct AnnaState {
    pub kv_store: KvStore,
    pub global_hash_ring: HashRing,
    pub local_hash_ring: HashRing,
    pub socket_pool: ZMQSocketCache,
    pub server_thread: ServerThread,
}

pub struct AnnaEventLoop<'a> {
    state: AnnaState,
    poll_items: Vec<PollItem<'a>>,
    handlers: Vec<(ZMQWrapper, dyn Handler)>
}

impl AnnaEventLoop {
    #[inline]
    pub fn new(state: AnnaState) -> AnnaEventLoop {
        AnnaEventLoop {
            state,
            poll_items: vec![],
            handlers: vec![]
        }
    }

    pub fn register(&mut self, socket: ZMQWrapper, pool_item: PollItem, handler: impl Handler) {
        self.poll_items.push(pool_item);
        self.handlers.push((socket, handler));
    }

    pub fn start(&mut self) {
        loop {
            let poll_result = zmq::poll(&mut self.poll_items, -1)
                .expect("Error while polling!");

            if poll_result <= 0 {
                continue;
            }

            self.poll_items.iter().enumerate()
                .filter(|(_, &p)| p.get_revents() == PollEvents::POLLIN)
                .for_each(|(i , _)| {
                    let (socket, handler) = self.handlers.get_mut(i).expect();
                    handler.handle(socket, &mut self.state)
                })
        }
    }
}

