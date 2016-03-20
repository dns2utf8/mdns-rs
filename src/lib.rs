/// Implementation of https://tools.ietf.org/html/rfc6762

pub struct Mdns {
    socket_timeout : u64,
    cached_descriptors : Vec<Descriptor>,
    published_descriptors : Vec<Descriptor>,
}

pub struct Descriptor {
    name : String, 
}

impl Mdns {
    pub fn new() -> Mdns {
        Mdns {
            socket_timeout : 30000, // 30 Sekunden
            cached_descriptors : vec![],
            published_descriptors : vec![],
        }
    }
    
    pub fn lookup_name(&self, name : &str) {
        
    }
    
    pub fn publish(&mut self, descriptor : Descriptor) {
        self.published_descriptors.push( descriptor );
    }
    
    pub fn publish_unique(&mut self, descriptor : Descriptor) {
        // TODO probe first
        self.publish(descriptor);
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        // TODO send 10.1. Goodbye Packets
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
