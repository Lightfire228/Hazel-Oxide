pub struct Application {
}


pub trait Run {
    fn run(&self) {
        println!(">>>> Entering main loop");
        
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


