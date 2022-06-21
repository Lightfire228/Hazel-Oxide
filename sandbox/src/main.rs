mod sandbox_app;
mod macros;

use hazel_oxide::application::Run;
use hazel_oxide::hlog;

use sandbox_app::Sandbox;

use log;

fn main() {

    hlog::init();

    log_info!("hiya 2");

    let sandbox: Box<Sandbox> = Box::new(Sandbox::new());

    sandbox.run();

}

