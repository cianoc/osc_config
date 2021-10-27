use serde::Deserialize;
use ron::de::from_reader;
use rosc::encoder;
use rosc::{OscMessage, OscBundle, OscPacket, OscType, OscTime};
use std::collections::HashMap;
use std::fs::File;
use std::net::{UdpSocket};
use std::time::{SystemTime};
use std::convert::TryFrom;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;


#[derive(Deserialize)]
struct Config{
    local_port: u16,
    address: String,
    port: u16,
    synths: HashMap<String, Vec<(String, f32)>>,
}

impl Config{
    fn get_msg_buf(&self) -> OscPacket{
        let mut messages: Vec<OscPacket> = Vec::new();
        for name in self.synths.keys(){
            for param in self.synths.get(name).unwrap(){
                let mut args = Vec::new();
                args.push(OscType::Float(param.1));
        
                messages.push(OscPacket::Message(OscMessage{
                    addr: format!{"/{}/{}", name, &param.0},
                    args: args,
                }));
            }    

        }
        OscPacket::Bundle(OscBundle{
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: messages,
        })
    }
}

fn get_config(file_name: &str) -> Config{
    let file = match File::open(file_name){
        Ok(result) => result,
        Err(_) => {
            println!{"Unable to find config file."};
            std::process::exit(1);
        }
    };
    
    match from_reader (file){
        Ok(x) => x,
        Err(e) => {
            println!(" Failed to read config file: {}", e);
            std::process::exit(1);
        }
    }

}

fn send_osc(config: Config){
    let sock = UdpSocket::bind(("127.0.0.1", config.local_port)).unwrap();

    let msg_buf = encoder::encode(&config.get_msg_buf()).unwrap();
    sock.send_to(&msg_buf, (config.address, config.port)).unwrap();

}

fn main() {

    let file_name = match std::env::args().nth(1) {
        Some (result) => result,
        None => {
            println!{"Please provide a config file."};
            std::process::exit(1);
        }
    };

    send_osc(get_config(&file_name));

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher.watch(&file_name, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
           Ok(event) => {
               match event {
                   DebouncedEvent::NoticeWrite(_) | DebouncedEvent::Create(_) 
                         => send_osc(get_config(&file_name)),
                   _ => {}
               }},
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
