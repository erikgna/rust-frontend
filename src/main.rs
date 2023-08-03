use yew::prelude::*;
use yew_router::prelude::*;

pub mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

fn switch(routes: Route) -> Html{
    match routes {
        Route::Home => html!{ <pages::home::Home /> },
        Route::Login => html!{ <pages::login::Login /> },
        Route::Register => html!{ <pages::register::Register /> },
    }
}

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