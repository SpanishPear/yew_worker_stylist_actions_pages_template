use gloo_worker::{HandlerId, Worker, WorkerScope};
use serde::{Deserialize, Serialize};

/// A struct containing
/// state for the Worker
pub struct MyWorker {}

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize)]
pub enum ToWorker {
    Ping,
}

/// Used for internal messaging
/// between the worker and itself
pub enum Message {
    Pong,
}

/// The type that a Worker
/// can send back
#[derive(Serialize, Deserialize, Debug)]
pub enum FromWorker {
    Pong(String),
}

impl Worker for MyWorker {
    type Input = ToWorker;
    type Message = ();
    type Output = FromWorker;

    fn create(scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        // no messaging in the example
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!
        let output = Self::Output::Pong("hello from worker".to_string());
        scope.respond(id, output);
    }
}
