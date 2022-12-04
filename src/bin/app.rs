// due to a bug in stylist
#![allow(clippy::let_unit_value)]

use bounce::BounceRoot;
use stylist::yew::*;
use yew::prelude::*;
use yew_stylist_actions_worker_template::components::app::App;

#[styled_component(AppRoot)]
fn app() -> Html {
    html! {
        <BounceRoot>
            <App />
        </BounceRoot>
    }
}

fn main() {
    yew::start_app::<AppRoot>();
    wasm_logger::init(wasm_logger::Config::default());
}
