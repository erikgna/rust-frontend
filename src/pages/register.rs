use validator::Validate;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, models::register::RegisterForm};

#[function_component(Register)]
pub fn register() -> Html {
    let first_name_input_node_ref = use_node_ref();
    let last_name_input_node_ref = use_node_ref();
    let email_input_node_ref = use_node_ref();
    let password_input_node_ref = use_node_ref();
    let confirm_password_input_node_ref = use_node_ref();

    let form = use_state(|| RegisterForm::default());

    let error_message = use_state(String::default);
    let error_message_value = (*error_message).clone();    

    let on_first_name_change = {
        let first_name_input_node_ref = first_name_input_node_ref.clone();        
        let cloned_form = form.clone();
        Callback::from(move |_| {
            let input = first_name_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                cloned_form.set(
                    RegisterForm {
                        first_name: input.value(),
                        last_name: cloned_form.last_name.clone(),
                        email: cloned_form.email.clone(),
                        password: cloned_form.password.clone(),
                        confirm_password: cloned_form.confirm_password.clone(),
                    }
                );
            }
        })
    };

    let on_last_name_change = {
        let last_name_input_node_ref = last_name_input_node_ref.clone();        
        let cloned_form = form.clone();
        Callback::from(move |_| {
            let input = last_name_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                cloned_form.set(
                    RegisterForm {
                        first_name: cloned_form.first_name.clone(),
                        last_name: input.value(),
                        email: cloned_form.email.clone(),
                        password: cloned_form.password.clone(),
                        confirm_password: cloned_form.confirm_password.clone(),
                    }
                );
            }
        })
    };

    let on_email_change = {
        let email_input_node_ref = email_input_node_ref.clone();        
        let cloned_form = form.clone();
        Callback::from(move |_| {
            let input = email_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                cloned_form.set(
                    RegisterForm {
                            first_name: cloned_form.first_name.clone(),
                            last_name: cloned_form.last_name.clone(),
                            email: input.value(),
                            password: cloned_form.password.clone(),
                            confirm_password: cloned_form.confirm_password.clone(),
                        }
                );
            }
        })
    };

    let on_password_change = {
        let password_input_node_ref = password_input_node_ref.clone();        
        let cloned_form = form.clone();
        Callback::from(move |_| {
            let input = password_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                cloned_form.set(
                    RegisterForm {
                        first_name: cloned_form.first_name.clone(),
                        last_name: cloned_form.last_name.clone(),
                        email: cloned_form.email.clone(),
                        password: input.value(),
                        confirm_password: cloned_form.confirm_password.clone(),
                    }
                );
            }
        })
    };

    let on_confirm_password_change = {
        let confirm_password_input_node_ref = confirm_password_input_node_ref.clone();        
        let cloned_form = form.clone();
        Callback::from(move |_| {
            let input = confirm_password_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                cloned_form.set(
                    RegisterForm {
                        first_name: cloned_form.first_name.clone(),
                        last_name: cloned_form.last_name.clone(),
                        email: cloned_form.email.clone(),
                        password: cloned_form.password.clone(),
                        confirm_password: input.value(),
                    }
                );
            }
        })
    };

    let navigator = use_navigator().unwrap();
    let on_submit = {
        let cloned_navigator = navigator.clone();
        let cloned_error_message = error_message.clone();        
        let cloned_form = form.clone();                    

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            match cloned_form.validate() {
                Ok(_) => {},
                Err(_) => {
                    cloned_error_message.set("Email ou senha incorretos".to_string());
                    return;
                }                                        
            }

            let navigator = cloned_navigator.clone();
            let error_message = cloned_error_message.clone();
            let form = cloned_form.clone();

            wasm_bindgen_futures::spawn_local(async move {                
                let map = form.to_hashmap().clone();                

                let client = reqwest::Client::new();
                let res = client.post("http://localhost:8000/api/v1/register")
                    .json(&map)
                    .send()
                    .await;

                match res {
                    Ok(_) => {
                        navigator.push(&Route::Home);
                    }
                    Err(_) => {
                        error_message.set("Ocorreu um erro, por favor, confira os dados".to_string());            
                    }
                }        
            });            
        })
    };

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
            <form onsubmit={on_submit} class={classes!("w-1/2", "px-8", "flex", "flex-col", "justify-center", "items-start")}>
                <h1 class={classes!("text-3xl", "font-bold", "mb-3")}>{ "Posturalle" }</h1>
                <p class={classes!("text-sm", "text-gray-600", "mb-6")}>{ "Acesse sua conta para agendar uma consulta" }</p>

                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="first_name" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Primeiro nome"}</label>
                    <input ref={first_name_input_node_ref} onchange={on_first_name_change} type="text" id="first_name" name="first_name" placeholder="Digite seu primeiro nome" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="last_name" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Último nome"}</label>
                    <input ref={last_name_input_node_ref} onchange={on_last_name_change}  type="text" id="last_name" name="last_name" placeholder="Digite seu último nome" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="email" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Email"}</label>
                    <input ref={email_input_node_ref} onchange={on_email_change}  type="email" id="email" name="email" placeholder="Digite seu email" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="password" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Senha"}</label>
                    <input ref={password_input_node_ref} onchange={on_password_change}  type="password" id="password" name="password" placeholder="Digite sua senha" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="confirm_password" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Confirmar senha"}</label>
                    <input ref={confirm_password_input_node_ref} onchange={on_confirm_password_change} type="password" id="confirm_password" name="confirm_password" placeholder="Digite a confirmação" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                if error_message_value.len() > 0 {
                    <p class={classes!("text-red-500", "mb-2", "text-sm")}>{ error_message_value }</p>
                }                
                                
                <button type="submit" class={classes!("bg-blue-500", "text-white", "font-semibold", "py-2", "px-4", "rounded", "hover:bg-blue-600", "focus:outline-none", "focus:ring", "focus:border-blue-300", "max-w-[700px]", "w-full")}>{ "Registrar" }</button>

                <p class={classes!("text-sm", "mt-2")}>
                    { "Já tem uma conta? "}
                    <Link<Route> to={Route::Login} classes="text-blue-500 font-semibold">{ "Entre aqui" }</Link<Route>>                    
                </p>
            </form>                              
        </section>
    }
}