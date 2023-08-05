use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route, switch};

mod pages;
mod routes;
mod models;

#[function_component(App)]
fn app() -> Html {
    html!{
        <BrowserRouter>
            <Switch <Route> render={switch} />        
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}