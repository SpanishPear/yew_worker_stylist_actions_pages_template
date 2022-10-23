// due to a bug in stylist
#![allow(clippy::let_unit_value)]

use bounce::BounceRoot;
use mipsy_web::components::app::App;
use stylist::yew::*;
use yew::prelude::*;

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
