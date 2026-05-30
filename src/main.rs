use std::{ io};
mod store;
mod command;
use command::Command;
use crate::store::Store;

fn main() {
    

    let mut store = Store::new();

    loop {
        let mut command_input = String::new();
        io::stdin()
            .read_line(&mut command_input)            
            .expect("Something went wrong");
    
        let command_input = command_input.to_uppercase();

        let parsed_command = Command::parse(&command_input);

        parsed_command.execute(&mut store);        
           
       

    }
}

