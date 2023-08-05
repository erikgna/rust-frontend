use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/forgot-password")]
    ForgotPassword,
}

pub fn switch(routes: Route) -> Html{
    match routes {
        Route::Home => html!{ <pages::home::Home /> },
        Route::Login => html!{ <pages::login::Login /> },
        Route::Register => html!{ <pages::register::Register /> },
        Route::ForgotPassword => html!{ <pages::forgot_password::ForgotPassword /> },
    }
}