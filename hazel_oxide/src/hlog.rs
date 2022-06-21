
use log4rs;

pub fn init() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}


pub const CORE:   &str = "core";
pub const CLIENT: &str = "client";