use hazel_oxide::application::Application;

pub struct Sandbox {
    pub parent: Box<Application>
}

impl Sandbox {
    pub fn run(&self) {
        self.parent.run();
    }

    pub fn new() -> Sandbox {
        Sandbox {
            parent: Box::new(Application::new()),
        }
    }
}