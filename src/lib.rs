/// Implementation of https://tools.ietf.org/html/rfc6762

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Mdns {
    socket_timeout : Duration,
    cached_descriptors : Vec<Descriptor>,
    published_descriptors : Vec<Descriptor>,
}

#[derive(Debug, Clone)]
pub struct Descriptor {
    name : String,
    timeout : Instant,
}

impl Mdns {
    pub fn new() -> Mdns {
        Mdns {
            socket_timeout : Duration::from_secs(30), // 30 Sekunden
            cached_descriptors : vec![],
            published_descriptors : vec![],
        }
    }
    
    pub fn lookup_name(&mut self, name : &str) -> Vec<Descriptor> {
        self.filter_timedout();
        
        self.cached_descriptors.iter().filter(|&e| {
            e.name == name
        }).cloned().into_iter().collect()
    }
    
    pub fn publish(&mut self, descriptor : Descriptor) {
        self.published_descriptors.push( descriptor );
    }
    
    pub fn publish_unique(&mut self, descriptor : Descriptor) {
        // TODO probe first
        self.publish(descriptor);
    }
    
    fn filter_timedout(&mut self) {
        let now = Instant::now();
        self.cached_descriptors.retain(|ref e| e.timeout < now);
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        // TODO send 10.1. Goodbye Packets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use std::thread;
    
    #[test]
    fn drop_out_cache() {
        let mut md = Mdns {
            socket_timeout : Duration::from_millis(0),
            cached_descriptors : vec![
                Descriptor{
                    name: "bye-bye.local".into(),
                    timeout : Instant::now(),
                }
            ],
            published_descriptors : vec![],
        };
        
        let r1 = md.lookup_name("bye-bye.local");
        assert_eq!( 1, r1.len() );
        
        thread::sleep(Duration::from_millis(23));
        
        let r0 = md.lookup_name("bye-bye.local");
        assert!( r0.is_empty() );
    }
    
    #[test]
    fn listen_to_published() {
        
    }
}
