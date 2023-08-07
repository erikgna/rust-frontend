use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::{Route, switch}, components::navbar::Navbar};

mod pages;
mod routes;
mod models;
mod components;

#[function_component(App)]
fn app() -> Html {
    html!{
        <BrowserRouter>
            <Navbar />
            <Switch <Route> render={switch} />        
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}