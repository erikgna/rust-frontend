use yew::prelude::*;
use yew_router::prelude::{Link, use_navigator};

use crate::routes::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let navigate_to_login = Callback::from(move |_: MouseEvent| navigator.push(&Route::Login));    

    html! {
        <nav class="bg-gray-700 shadow-lg">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <Link<Route> to={Route::Home} classes="flex-shrink-0 text-white font-bold text-xl">{ "My Website" }</Link<Route>>                            
                    </div>
                    <div class="flex">                        
                        <Link<Route> to={Route::Home} classes="text-gray-300 hover:bg-gray-800 px-3 py-2 rounded-md text-sm font-medium">{ "Início" }</Link<Route>>
                        <Link<Route> to={Route::Register} classes="text-gray-300 hover:bg-gray-800 px-3 py-2 rounded-md text-sm font-medium">{ "Sobre nós" }</Link<Route>>
                        <Link<Route> to={Route::Register} classes="text-gray-300 hover:bg-gray-800 px-3 py-2 rounded-md text-sm font-medium">{ "Contato" }</Link<Route>>
                        
                        <button onclick={navigate_to_login} class={classes!("text-sm", "font-semibold", "bg-white", "text-gray-700", "px-2", "hover:bg-blue-500", "hover:text-white", "focus:outline-none", "focus:ring", "focus:border-blue-300", "mx-2", "transition-all", "duration-300")}>{ "Acessar conta" }</button>
                        <button class={classes!("text-sm", "font-semibold", "text-white", "bg-transparent", "border", "border-white", "px-2", "hover:text-blue-500", "hover:border-blue-500", "focus:outline-none", "focus:ring", "focus:border-blue-300", "transition-all", "duration-300")}>{ "Agende aqui" }</button>                        
                    </div>
                </div>
            </div>
        </nav>
    }
}