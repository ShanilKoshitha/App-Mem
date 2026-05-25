use std::{collections::HashMap, io, process::{self, exit}};

enum Command{
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    List,
    Clear,
    Exit,
    Unknown
}

fn main() {
    
    
    let mut fast_store:HashMap<String, String> = HashMap::new();
    
    loop {
        let mut command_input = String::new();
        io::stdin()
            .read_line(&mut command_input)
            .expect("Something went wrong");
    
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

    fn execute(&self, map: &mut HashMap<String, String>){
        
        match self {
            Command::Set(key, value) => {
                map.insert(key.to_string(), value.to_string());
                println!("OK");
                false;
            }
            Command::Get(key) => {
                map.get(key);
                println!("OK");
                false;
            },
            Command::Delete(key) => {
                map.remove(&key.to_string());
                println!("OK");
                false;
            },
            Command::Exists(key) => {
                map.contains_key(&key.to_string());
                println!("OK");
                false;
            },
            Command::List => {
                for (key, val) in map.iter(){
                    println!("Key : {key} Value: {val}");
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
        }
    }
}