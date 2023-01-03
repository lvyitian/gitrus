use std::{fs::File, io::Write};

use serde::{Serialize, Deserialize};
#[derive(Debug,Serialize, Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub database: Option<Database>
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Database {
    pub dbtype: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>
}
pub fn check_config(file:&mut File,config:&mut Config)
{
    let mut msg=vec![];
    if config.port.is_none() {
        msg.push("port");
        config.port=Some(80u16);
    }
    let default_database=Database{dbtype:None,host:None,port:None};
    if config.database.is_none() {
        msg.push("database");
        config.database=Some(default_database);
    }

    if config.database.as_ref().unwrap().dbtype.is_none() {
        msg.push("database.dbtype");
        config.database.as_mut().unwrap().dbtype=Some(String::from("mongodb"));
    }
    if config.database.as_ref().unwrap().host.is_none() {
      msg.push("database.host");
      config.database.as_mut().unwrap().host=Some(String::from("127.0.0.1"));
    }
    if config.database.as_ref().unwrap().port.is_none() {
      msg.push("database.port");
      config.database.as_mut().unwrap().port=Some(27017u16);
    }
    if !msg.is_empty() {
        file.write(toml::to_string(config).unwrap().as_bytes()).unwrap();
        panic!("{:?} cannot be empty, new config was generated, panic.",msg);
    }
}
