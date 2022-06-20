mod sandbox_app;

use hazel_oxide::application::Run;

use sandbox_app::Sandbox;


fn main() {

    let sandbox: Box<Sandbox> = Box::new(Sandbox::new());

    sandbox.run();

}
