mod sandbox_app;

use sandbox_app::Sandbox;

fn main() {

    let sandbox: Box<Sandbox> = Box::new(Sandbox::new());

    sandbox.run();

}
