use gloo_worker::Registrable;
use yew_stylist_actions_worker_template::agent::MyWorker;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    MyWorker::registrar().register();
}
