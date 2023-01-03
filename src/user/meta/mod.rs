use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct UserInfo{
    uid:u64,
    name:String
}

impl UserInfo{

}
