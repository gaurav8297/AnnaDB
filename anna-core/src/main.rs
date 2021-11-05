use crate::lattice::Lattice;
use std::collections::HashMap;

pub mod lattices;
mod server_thread;
mod zmq;
mod kv_store;
mod hash_ring;
mod event_loop;
mod handlers;
mod init_handler;


use crate::lattices::base_lattices as lattice;
use crate::init_handler::InitHandler;
use std::thread;
use crate::server_thread::ServerThread;
use std::thread::JoinHandle;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::Environment::with_prefix("annadb")).unwrap();

    let thread_count: usize = settings.get("thread_count").unwrap();
    let seed_ip: String = settings.get("seed_ip").unwrap();
    let public_ip: String = settings.get("public_ip").unwrap();
    let private_ip: String = settings.get("private_ip").unwrap();

    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..thread_count {
        let handle = thread::spawn(|| {
            let init_handler = InitHandler {
                server_thread: ServerThread {
                    public_ip: public_ip.clone(),
                    private_ip: private_ip.clone(),
                    thread_id: i,
                    virtual_num: 0
                },
                thread_count,
                seed_ip: seed_ip.clone(),
                is_seed_node: false
            };

            let mut event_loop = init_handler.init();
            event_loop.start();
        });

        thread_handles.push(handle);
    }

    thread_handles.iter().for_each(|handle| {handle.join();});
}
