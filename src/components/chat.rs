use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

const PROFILE_PIC: &str = "https://i.pinimg.com/736x/a6/bb/e4/a6bbe48d62785cd6ded78158470ea0ad.jpg";

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    username: String,
    messages: Vec<String>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        Self {
            users: vec![UserProfile {
                name: username.clone(),
                avatar: PROFILE_PIC.into(),
            }],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            username,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                self.messages.push(s);
                true
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message_text = input.value();
                    if message_text.is_empty() {
                        return false;
                    }

                    if let Err(e) = self.wss.tx.clone().try_send(message_text) {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                    return true;
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen bg-slate-900 text-slate-100">
                <div class="flex-none w-56 h-screen bg-slate-800/90 border-r border-slate-700">
                    <div class="text-xl p-3 text-cyan-300">{"Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 rounded-lg p-2 bg-slate-700/60 border border-slate-600">
                                    <div>
                                        <img class="w-12 h-12 rounded-full border border-cyan-300/40 object-cover" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between text-white">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-slate-300">
                                            {"Hi there!"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col bg-slate-900">
                    <div class="w-full h-14 border-b border-slate-700 bg-slate-800/70"><div class="text-xl p-3 text-cyan-300">{"💬 Chat!"}</div></div>
                    <div class="w-full grow overflow-auto border-b border-slate-700">
                        {
                            self.messages.iter().map(|m| {
                                html!{
                                    <div class="flex items-end w-3/6 bg-slate-800 m-8 rounded-tl-lg rounded-tr-lg rounded-br-lg border border-slate-700">
                                        <img class="w-8 h-8 rounded-full m-3 border border-cyan-300/40 object-cover" src={PROFILE_PIC} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm text-white">
                                                {self.username.clone()}
                                            </div>
                                            <div class="text-xs text-slate-300">
                                                {m.clone()}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }

                    </div>
                    <div class="w-full h-14 flex px-3 items-center bg-slate-900">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-2 pl-4 mx-3 bg-slate-800 text-white rounded-full outline-none border border-slate-700 focus:text-white focus:border-cyan-400" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm bg-cyan-500 w-10 h-10 rounded-full flex justify-center items-center text-white border border-cyan-300/30">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
