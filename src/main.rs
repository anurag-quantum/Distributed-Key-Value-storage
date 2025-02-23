
//step 1
//Create basic key hash map HashMap<String,String>
//make it global
//methods : put ,get ,delete

//How users interact
//1.  CLI    
// 2. we will add network support later.




use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::map::Keys;
use std::sync::Mutex;



use std:: {
    io::{prelude:: *, BufReader},
    net::{TcpListener,TcpStream},
};


#[macro_use]
extern crate lazy_static;


#[derive(Debug, Serialize, Deserialize)]
struct Configuration{
    machine_name: String,
    host_ip     : String,
    machine_type: String,
    profile_id  : u64,

}

lazy_static::lazy_static! {
    static ref MACHINE_CONFIGS: Mutex<HashMap<String, Configuration>> = Mutex::new(HashMap::new());
}
 


fn load_db(filepath: &str) {
    let file_content = fs::read_to_string(filepath)
                        .unwrap_or_else(|_|"{}".to_string());

    let mut configs = MACHINE_CONFIGS.lock().unwrap();

    if !file_content.is_empty() {
        println!("Loading Existing Entries!!");
        *configs = serde_json::from_str(&file_content)
                .expect("Failed to parse JSON");


    }
    else{
        println!("No Entries found in db file!! ");
        *configs = HashMap::new();
    }



}
fn process_input(){

    loop{
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str>  = input.trim().split_whitespace().collect();

        if parts.is_empty(){
            continue;
        }

        match parts[0] {
            "put" if parts.len() == 6  => {
                
                    let Key = parts[1].to_string();
                    let Value = Configuration{
                            machine_name: parts[2].to_string(),
                            host_ip : parts[3].to_string(),
                            machine_type: parts[4].to_string(),
                            profile_id: parts[5].parse().unwrap_or(0),
                    };
                    println!("Entries {:?}",parts);
                    put(Key, Value);
                
            }
            "get" if parts.len() == 2 =>{
                let Key = parts[1].to_string();
                get(&Key);
            }
            "del" if parts.len() == 2 =>{
                let Key = parts[1].to_string();
                del(&Key);
            }
            "exit" =>{
                break;
            }
            _ => println!("Invalid command! Use: put <key> <name> <ip> <type> <id> | get <key> | delete <key> | exit"),
            
        }
    }
}

fn save_data(){
    let configs = MACHINE_CONFIGS.lock().unwrap();
    let json = serde_json::to_string_pretty(&*configs).expect("Failed to serialize JSON");
    fs::write("config_db.txt", json).expect("Failed to write to file");
}

fn put(Key: String, Value: Configuration){
    //converts to Json
    {
        let mut configs = MACHINE_CONFIGS.lock().unwrap();
        configs.insert(Key.clone(), Value);
    }
    save_data();
    println!("✅ Entry '{}' added successfully!", Key);
}

fn get(Key: &String) {
    //extracts the key value for 
    let mut configs = MACHINE_CONFIGS.lock().unwrap();
    match configs.get(Key) {
        Some(value) => println!("Found! {:?}",value),
        None => println!("Not found!!"),
    }
}

fn del(Key: &String){
    let mut is_present = false;

    {
      let mut configs = MACHINE_CONFIGS.lock().unwrap();
      is_present = configs.remove(Key).is_some();
    }
    if is_present == true {
        println!("🗑️  Deleted key '{}'", Key);
        save_data();
    } else {
        println!("❌ Key '{}' not found!", Key);
    }
}

fn handle_connection(mut stream : TcpStream){
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader.lines()
                                .map(|result| result.unwrap())
                                .take_while(|line| !line.is_empty())
                                .collect();
    println!("Request: {http_request:#?}");

    //contents of HTTP response
    let status = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("response.html").unwrap();
    let length = contents.len();
    
    //Formating the HTTP response
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");


    stream.write_all(response.as_bytes()).unwrap();


}

fn main(){
    load_db("config_db.txt");
    

    let mut listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    // process_input();

    println!("Let's make distributed key value store by tonight!!");

}
