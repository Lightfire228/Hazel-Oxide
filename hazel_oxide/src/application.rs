
pub struct Application {

}

impl Application {
    pub fn new() -> Application {
        Application {
    
        }
    }

    pub fn run(&self) {
        println!(">>>> Entering main loop");
        
        #[allow(while_true)]
        while true {};
    }
}

