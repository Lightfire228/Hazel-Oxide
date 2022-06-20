use hazel_oxide::application::{Application, Run};

pub struct Sandbox {
    pub parent: Box<Application>
}

impl Run for Sandbox {}

impl Sandbox {
    
    pub fn new() -> Sandbox {
        Sandbox {
            parent: Box::new(Application::new()),
        }
    }
}