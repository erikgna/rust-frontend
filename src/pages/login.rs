use std::collections::HashMap;
use log::info;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
struct Video {
    id: usize,
    title: String,
    author: String,    
}

#[function_component(Login)]
pub fn login() -> Html {
    let email_input_node_ref = use_node_ref();
    let email_input_value_handle = use_state(String::default);
    let email_input_value = (*email_input_value_handle).clone();

    let password_input_node_ref = use_node_ref();
    let password_input_value_handle = use_state(String::default);
    let password_input_value = (*password_input_value_handle).clone();

    let email_val = (*email_input_value_handle).clone();
    let password_val = (*password_input_value_handle).clone();

    let on_email_change = {
        let email_input_node_ref = email_input_node_ref.clone();        
        Callback::from(move |_| {
            let input = email_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                email_input_value_handle.set(input.value());
            }
        })
    };

    let on_password_change = {
        let password_input_node_ref = password_input_node_ref.clone();        
        Callback::from(move |_| {
            let input = password_input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {                
                password_input_value_handle.set(input.value());
            }
        })
    };

    let onclick = Callback::from(move |_| {        
        let email = email_input_value.clone();
        let password = password_input_value.clone();
        wasm_bindgen_futures::spawn_local(async move {            
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("email", email);            
            map.insert("password", password);
    
            let client = reqwest::Client::new();
            let res = client.post("http://localhost:8000/api/v1/login")
                .json(&map)
                .send()
                .await;
    
            let json_response = res.unwrap().text().await.unwrap();
    
            info!("{}", json_response);
        });
    });

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
                <p class={classes!("text-sm", "text-gray-600", "mb-6")}>{ "Acesse sua conta para agendar uma consulta" }</p>

                <div class={classes!("mb-4", "w-full", "max-w-[700px]")}>
                    <label for="email" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Email"}</label>
                    <input ref={email_input_node_ref} onchange={on_email_change}  type="email" id="email" name="email" placeholder="Digite seu email" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>
                <div class={classes!("mb-2", "w-full", "max-w-[700px]")}>
                    <label for="password" class={classes!("block", "text-sm", "font-semibold", "mb-2")}>{"Senha"}</label>
                    <input ref={password_input_node_ref} onchange={on_password_change} type="password" id="password" name="password" placeholder="Digite sua senha" class={classes!("w-full", "px-3", "py-2", "border", "rounded", "focus:outline-none", "focus:ring", "focus:border-blue-300")} />
                </div>

                <a href="#" class={classes!("text-blue-500", "text-sm", "mb-6")}>{ "Esqueceu sua senha?" }</a>

                <button onclick={onclick} type="button" class={classes!("bg-blue-500", "text-white", "font-semibold", "py-2", "px-4", "rounded", "hover:bg-blue-600", "focus:outline-none", "focus:ring", "focus:border-blue-300", "max-w-[700px]", "w-full")}>{ "Entrar" }</button>                

                <p class={classes!("text-sm", "mt-2")}>
                    { "Ainda não tem uma conta? "}
                    <a href="#" class={classes!("text-blue-500", "font-semibold")}>{ "Cadastre-se aqui" }</a>
                </p>
            </form>                              
        </section>
    }
}