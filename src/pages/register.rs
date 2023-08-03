use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <section class={classes!("flex", "h-screen")}>
            <div class={classes!("w-1/2", "bg-gray-700", "flex", "flex-col", "justify-center", "px-6")}>
                <h1 class={classes!("text-4xl", "font-semibold", "mb-4", "text-white")}>{ "O Melhor Espaço Para Se Cuidar" }</h1>
                <p class={classes!("text-sm", "text-white", "mb-8")}>{ "Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like)." }</p>
                <div class={classes!("flex")}>
                    <button class={classes!("font-semibold", "bg-white", "text-gray-700", "px-4", "py-2", "hover:bg-blue-500", "hover:text-white", "focus:outline-none", "focus:ring", "focus:border-blue-300", "mr-4", "transition-all", "duration-300")}>{ "Aprenda sobre nós" }</button>
                    <button class={classes!("font-semibold", "text-white", "bg-transparent", "border", "border-white", "px-4", "py-2", "mr-4", "hover:text-blue-500", "hover:border-blue-500", "focus:outline-none", "focus:ring", "focus:border-blue-300", "transition-all", "duration-300")}>{ "Ver agenda" }</button>
                </div>
            </div>
            <form class={classes!("w-1/2", "px-8", "flex", "flex-col", "justify-center", "items-start")}>
                <h1 class={classes!("text-3xl", "font-bold", "mb-3")}>{ "Posturalle" }</h1>
                <p class={classes!("text-sm", "text-gray-600", "mb-6")}>{ "Cria uma conta para poder agendar consultas" }</p>

                <div class={classes!("mb-2", "w-full", "max-w-[700px]")}>
                    <label for="first_name" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Primeiro nome"}</label>
                    <input type="text" id="first_name" name="first_name" placeholder="Digite o seu primeiro nome" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                <div class={classes!("mb-2", "w-full", "max-w-[700px]")}>
                    <label for="last_name" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Último nome"}</label>
                    <input type="text" id="last_name" name="last_name" placeholder="Digite o seu último nome" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                
                <div class={classes!("mb-2", "w-full", "max-w-[700px]")}>
                    <label for="email" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Email"}</label>
                    <input type="email" id="email" name="email" placeholder="Digite seu email" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                <div class={classes!("mb-2", "w-full", "max-w-[700px]")}>
                    <label for="password" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Senha"}</label>
                    <input type="password" id="password" name="password" placeholder="Digite sua senha" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="confirm_password" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Confirmar senha"}</label>
                    <input type="password" id="confirm_password" name="confirm_password" placeholder="Digite a confirmação" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                <button {onclick} class={classes!("bg-blue-500", "text-white", "font-semibold", "py-2", "px-4", "rounded", "hover:bg-blue-600", "focus:outline-none", "focus:ring", "focus:border-blue-300", "max-w-[700px]", "w-full")}>{ "Cadastrar" }</button>

                <p class={classes!("text-sm", "mt-2")}>
                    { "Já tem uma conta? "}
                    <a href="#" class={classes!("text-blue-500", "font-semibold")}>{ "Entre aqui" }</a>
                </p>
            </form>              
        </section>
    }
}