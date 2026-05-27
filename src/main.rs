use std::{collections::HashMap, io, process::{self, exit}, time::{Duration, SystemTime}};

enum Command{
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    SetEx(String, isize, String),
    List,
    Clear,
    Exit,
    Unknown
}

struct Entry {
    value: String,
    expires_at: Option<SystemTime>
}

fn main() {
    
    
    let mut fast_store:HashMap<String, Entry> = HashMap::new();
    
    loop {
        let mut command_input = String::new();
        io::stdin()
            .read_line(&mut command_input)            
            .expect("Something went wrong");
    
        let command_input = command_input.to_uppercase();
        //The format is : COMMAND KEY_NAME VALUE
    
        let command: Command = Command::parse(&command_input);
    
        command.execute( &mut fast_store);

    }
}

impl Command {

    fn parse(input: &str) -> Command{

        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.as_slice() {
            ["GET", key] => {
                Command::Get((*key).to_string())
            },
            ["SET", key, value] => {
                Command::Set((*key).to_string(), (*value).to_string())
            },
            ["DELETE", key] => {
                Command::Delete((*key).to_string())
            },
            ["EXISTS", key] => {
                Command::Exists((*key).to_string())
            },
            ["SETEX", key,seconds, value] => {
                Command::SetEx((*key).to_string(), seconds.parse::<isize>().unwrap(), value.to_string())
            },
            ["LIST"] => {
                Command::List
            },
            ["CLEAR"] => {
                Command::Clear
            },
            ["EXIT"] => {
                Command::Exit
            }
            _ => Command::Unknown
        }

    }

    fn execute(&self, map: &mut HashMap<String, Entry>){
        
        match self {
            Command::Set(key, value) => {
                self.execute_set(key,value,map);
            }
            Command::Get(key) => {                
                self.execute_get(key, map);
            },
            Command::Delete(key) => {
                self.execute_delete(key,map);
            },
            Command::Exists(key) => {
                self.execute_exists(key,map)
            },
            Command::List => {
                self.remove_expired(map);
                
                for (key, val) in map.iter(){
                    println!("Key : {key} Value: {}",val.value);
                }
                false;
            },
            Command::Clear => {
                map.clear();
                false;
            },
            Command::Exit => {
                map.clear();
                println!("Bye!");
                exit(0);                
            },
            Command::Unknown => {
                println!("Unknown Command");
            },
            Command::SetEx(key, seconds,value) => {
                map.insert(key.to_string(), 
                Entry {
                    value: value.to_string(), 
                    expires_at: Some(SystemTime::now() + Duration::from_secs((*seconds) as u64))
                });
            },
        }
    }

    fn execute_set(&self, key:&String, value: &String, map: &mut HashMap<String, Entry>){

        map.insert(key.clone(),                
            Entry {
                value: value.clone(),
                expires_at: None
            } 
        );
        println!("OK");
        false;
    }

    fn execute_get(&self, key:&String, map: &mut HashMap<String, Entry>){
        
        let is_expired = match map.get(key) {
            Some(value) =>{

                match value.expires_at {
                    Some(expires_at) => SystemTime::now() >= expires_at,
                    None=> false
                }
            }
            None => false
        };

        if is_expired {
            map.remove(key);
            println!("(nil)");
        }else{
            match map.get(key){
                Some(value) => println!("Value: {}", value.value),
                None=> println!("(nil)")
            }
        }
    }

    fn execute_delete(&self, key:&String, map: &mut HashMap<String, Entry>){
        map.remove(&key.to_string());
        println!("OK");
        false;
    }

    fn execute_exists(&self, key:&String, map: &mut HashMap<String, Entry>){
        let if_key_exists = map.contains_key(&key.to_string());
        if if_key_exists{
            println!("TRUE");
        }else{
            println!("FALSE");
        }
    }

    fn remove_expired(&self,map: &mut HashMap<String, Entry>){
        let time_now = SystemTime::now();

        map.retain(|_, entry| {
            match entry.expires_at {
                Some(expires_at) => expires_at > time_now,
                None => true,
            }
        });
    }
}