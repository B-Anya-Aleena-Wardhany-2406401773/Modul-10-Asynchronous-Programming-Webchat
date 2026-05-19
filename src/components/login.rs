use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-slate-900 flex w-screen min-h-screen text-white">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <form class="m-4 flex">
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-white border-slate-600 bg-slate-800 placeholder:text-slate-400" placeholder="Username" />
                    <Link<Route> to={Route::Chat}> <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-cyan-500 text-white font-bold p-4 uppercase border-cyan-400 border-t border-b border-r" >{"Go Chatting!"}</button></Link<Route>>
                </form>
            </div>
        </div>
    }
}
