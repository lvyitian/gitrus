use std::{fs::OpenOptions, sync::Mutex};
use std::io::Read;
use std::string::ToString;
use chrono::Local;
use simple_log::{info, LogConfigBuilder};
#[macro_use] extern crate lazy_static;

pub mod user;
pub mod config;
pub mod database;

lazy_static!{
    static ref SERVER_CONFIG:Mutex<config::Config>=Mutex::new(config::Config{port:None,database:None});
}

#[actix_web::main]
async fn main() {
    let mut file_name = "./log/".to_owned();
    file_name += &Local::now().format("%Y-%m-%d.%H-%M-%S").to_string();
    file_name += ".log";

    let config = LogConfigBuilder::builder()
        .path(&file_name)
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config).expect("Cannot init logger");

    info!("Initializing...");

    //read config.toml
    let mut server_config: config::Config;
    let mut config: String = Default::default();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.toml").expect("Can not open 'server.config.json'");
    file.read_to_string(&mut config).expect("Can not read 'config.toml'");
    server_config = toml::from_str(config.as_str()).expect("Can not parse 'config.toml' as json");
    // config::init(&mut server_config);
    config::check_config(&mut file,&mut server_config);
    *SERVER_CONFIG.lock().unwrap()=server_config;
}
