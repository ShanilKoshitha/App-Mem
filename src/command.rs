
use crate::{store::{Store}};

pub enum Command{
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    SetEx(String, u64, String),
    TTL(String),
    List,
    Clear,
    Exit,
    Unknown
}

impl Command{
    
    pub fn parse(input: &str) -> Command{

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
                Command::SetEx((*key).to_string(), seconds.parse::<u64>().unwrap(), value.to_string())
            },
            ["TTL", key] => {
                Command::TTL((*key).to_string())
            },
            ["LIST"] => {
                Command::List
            },
            ["CLEAR"] => {
                Command::Clear
            },
            ["EXIT"] => {
                Command::Exit
            },
            _ => Command::Unknown
        }

    }

    pub fn execute(&self, store: &mut Store){
        
        match self {
            Command::Set(key, value) => {
                store.set(key, value);
            }
            Command::Get(key) => {                
                store.get(key);
            },
            Command::Delete(key) => {
                store.delete(key);
            },
            Command::Exists(key) => {
                store.exists(key);
            },
            Command::List => {
                store.list();
            },
            Command::Clear => {
                store.clear();
            },
            Command::Exit => {
                store.exit();               
            },
            Command::Unknown => {
                println!("Unknown Command");
            },
            Command::SetEx(key, seconds,value) => {
                store.set_ex(key, *seconds, value);
            },
            Command::TTL(key)=>{
                store.ttl(key);
            }
        }
    }

    
}