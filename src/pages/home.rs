use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Login));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go Login" }</button>
        </div>
    }
}