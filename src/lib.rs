use std::time::SystemTime;


#[derive(Debug)]
pub struct MemCacheHeader{
    pub ttl:u32,
    pub key: String,
    pub created_at:u128,    
}
impl MemCacheHeader{
    fn new(key:&String, ttl:&u32)-> MemCacheHeader{
        let now:u128 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

        return MemCacheHeader {
            ttl:ttl.to_owned(),
            key: key.to_owned(),
            created_at:now
        }
    }
}


#[derive(Debug)]
pub struct MemCache{
    pub header:MemCacheHeader,
    pub data:Vec<u8>
}


impl MemCache {
    pub fn new(key:&String, data:Vec<u8>, ttl:&u32)-> MemCache{
        return MemCache {
            header: MemCacheHeader::new(key, ttl),
            data
        }
    }
}

pub fn find_data_by_key(array: &[MemCache], key: String) -> Option<&MemCache> {
    for item in array.iter() {
        if item.header.key == key {
            return Some(item);
        }
    }
    None
}