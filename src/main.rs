extern crate reqwest_wasm;

use yew::prelude::*;

enum Msg {
    FetchNewUser,
}

struct App;
    
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::FetchNewUser => {
                // link.send_future(App::get_person());

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::FetchNewUser);

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-col items-center justify-center gap-y-4">
                <div class="w-96 h-80 bg-gray-800 shadow-md border-indigo-400 h-auto w-auto p-4 pl-8 pr-8 rounded-md font-medium text-xl flex flex-col items-center">
                    // <img class="rounded-full w-24 h-24" src={self.user.picture.large.to_string()} />
                    <div class="mt-4 mb-4 flex flex-col gap-y-1">
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Gender: "}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Registered: "}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Phone number: "}</span>
                        </div>
                    </div>
                </div>

                <button {onclick} class="bg-indigo-400 shadow-md h-auto w-auto pl-4 pr-4 pb-2 pt-2 rounded-md font-medium text-xl text-white hover:bg-indigo-300">{"Find date"}</button>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // if first_render {
        //     ctx.link().send_future(App::get_person());
        // }
    }
}

// impl App {
//     async fn get_person() -> Msg {
//         let res = reqwest_wasm::get("https://randomuser.me/api/1.2")
//             .await
//             .unwrap()
//             .text()
//             .await
//             .unwrap();

//         let parsed_json = serde_json::from_str::<data::Root>(res.as_str()).unwrap();
//         return Msg::UpdatePerson((*parsed_json.results.first().unwrap()).clone());
//     }
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
