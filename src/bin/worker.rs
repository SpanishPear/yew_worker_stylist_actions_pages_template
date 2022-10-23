use crate::agent::MipsyWebWorker;
use gloo_worker::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    MyWorker::registrar().register();
}
