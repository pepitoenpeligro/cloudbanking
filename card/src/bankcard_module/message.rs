use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
pub struct Msg {
    pub msg: String,
}

impl Msg{
    pub fn new() ->  Msg {
        Msg {
            msg: String::new()
        }
    }
}