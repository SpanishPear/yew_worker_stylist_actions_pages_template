use crate::agent::MyWorker;
use gloo_worker::Spawnable;
use js_sys::Promise;
use stylist::css;
use stylist::yew::styled_component;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let bridge = MyWorker::spawner()
        .callback(move |m| {
            // this runs in the main browser thread
            // and does not block the web worker
            log::info!("received message from worker: {:?}", m);
        })
        .spawn("/worker.js");

    spawn_local(async move {
        bridge.send(crate::agent::ToWorker::Ping);
        // We need to hold the bridge until the worker resolves.
        let promise = Promise::new(&mut |_, _| {});
        let a = JsFuture::from(promise).await;
        log::error!("{:?}", a);
    });

    html! {
        <AppContainer>
            <h1>{"Hello, world!"}</h1>
        </AppContainer>
    }
}

#[derive(Properties, PartialEq)]
pub struct AppContainerProps {
    pub children: Children,
}

#[styled_component(AppContainer)]
pub fn app_container(props: &AppContainerProps) -> Html {
    html! {
        <div class={css!(r#"
            min-width: 100vw;
            min-height: 100vh;
            height: 100%;
            width: 100%;
            background-color: pink;
        "#)}>
            {
                for props.children.iter()
            }
        </div>
    }
}
