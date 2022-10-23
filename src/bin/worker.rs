use crate::agent::MipsyWebWorker;
use gloo_worker::Registrable;

fn main() {
    //console_error_panic_hook::set_once();

    //wasm_logger::init(wasm_logger::Config::default());
    MyWorker::registrar().register();
}
