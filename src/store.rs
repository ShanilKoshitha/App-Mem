use std::{collections::HashMap, process::exit, time::{Duration, SystemTime}};

pub struct Entry{
    pub value: String,
    pub expires_at: Option<SystemTime>
}

pub struct Store{
    map: HashMap<String, Entry>
}

impl Store{
    pub fn new() -> Store {
        Store {
            map: HashMap::new()
        }
    }

    pub fn set(&mut self, key: &str, value: &str){
        self.map.insert(
            key.to_string(),
            Entry{
                value: value.to_string(),
                expires_at: None
            }
        );
    }

    pub fn set_ex(&mut self, key: &str, seconds:u64, value: &str){
        
        self.map.insert(
            key.to_string(),
            Entry { 
                value: value.to_string(), 
                expires_at: Some(SystemTime::now() + Duration::from_secs(seconds)) 
            }
        );
    }

    pub fn get(&mut self, key:&str){
        
        let is_expired = match self.map.get(key) {
            Some(value) =>{

                match value.expires_at {
                    Some(expires_at) => SystemTime::now() >= expires_at,
                    None=> false
                }
            }
            None => false
        };

        if is_expired {
            self.map.remove(key);
            println!("(nil)");
        }else{
            match self.map.get(key){
                Some(value) => println!("Value: {}", value.value),
                None=> println!("(nil)")
            }
        }
    }

    pub fn delete(&mut self, key:&str){
        self.map.remove(&key.to_string());
    }

    pub fn exists(&mut self, key:&String){
        let if_key_exists = self.map.contains_key(&key.to_string());
        if if_key_exists{
            println!("TRUE");
        }else{
            println!("FALSE");
        }
    }

    pub fn remove_expired(&mut self){
        let time_now = SystemTime::now();

        self.map.retain(|_, entry| {
            match entry.expires_at {
                Some(expires_at) => expires_at > time_now,
                None => true,
            }
        });
    }
    
    pub fn list(&mut self){
        self.remove_expired();
        //
        for (key, val) in self.map.iter(){
            println!("Key: {key} Value: {}", val.value);
        }
    }

    pub fn ttl(&mut self, key: &str){
       
       let is_expired = match self.map.get(key) {
            Some(value) =>{

                match value.expires_at {
                    Some(expires_at) => SystemTime::now() >= expires_at,
                    None=> false
                }
            }
            None => false
        };

        if is_expired {
            self.map.remove(key);
            println!("(nil)");
        }else{
            match self.map.get(key){
                Some(value) => match value.expires_at{
                    Some(expires_at)=> println!("Seconds: {:?}",expires_at.duration_since(SystemTime::now())),
                    None=> println!("(nil)")
                },
                None=> println!("(nil)")
            }
        }
        
    }

    pub fn clear(&mut self){
        self.map.clear();
    }

    pub fn exit(&mut self){
        self.clear();
        println!("Bye!");
        exit(0);
    }
}