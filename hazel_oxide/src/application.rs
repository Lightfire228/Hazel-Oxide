use crate::events::application_event::window_event::window_resized::WindowResizedEvent;
use crate::hlog;

use crate::log_trace;

pub struct Application {
}



pub trait Run {
    fn run(&self) {
        println!(">>>> Entering main loop");
        
        let e = WindowResizedEvent::new(1280, 720);
        log_trace!("{}", e.to_string());

        #[allow(while_true)]
        while true {};
    }
}

impl Run for Application {}

impl Application {
    pub fn new() -> Application {

        Application {
        }
    }
}


