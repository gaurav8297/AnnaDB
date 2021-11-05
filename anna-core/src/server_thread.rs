
pub struct ServerThread {
    pub public_ip: String,
    pub private_ip: String,
    pub thread_id: usize,
    pub virtual_num: usize,
}

impl ServerThread {
    pub fn virtual_id(&self) -> String {
        return format!("{}:{}/{}", self.private_ip, self.thread_id, self.virtual_num);
    }

    pub fn get_id(&self) -> String {
        return format!("{}:{}", self.private_ip, self.thread_id);
    }

    pub fn get_node_join_addr(&self) -> String {
        return format!("tcp://{}:5055", self.private_ip);
    }

    pub fn get_node_connect_addr(&self, private_ip: String) -> String {
        return format!("tcp://{}:5055", private_ip);
    }

    pub fn get_seed_connect_addr(&self, private_ip: String) -> String {
        return format!("tcp://{}:5057", private_ip);
    }

    pub fn get_seed_addr(&self) -> String {
        return format!("tcp://{}:5057", self.private_ip);
    }

    pub fn get_req_addr(&self) -> String {
        return format!("tcp://{}:5059", self.private_ip);
    }
}
