// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};
use futures::prelude::*;

// ------ ------
//     Model
// ------ ------

struct Model {
    content: String
}

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model{content: "Nothing".to_string()}
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    FetchGood,
    FetchBad,
    ContentReceived(String)
}

fn fetch_content(filename: &'static str) -> impl Future<Output = Msg> {
    async fn inner(filename: &'static str) -> fetch::Result<String> {
        fetch(filename).await?.check_status()?.text().await
    }
    inner(filename).map(|result| {
        Msg::ContentReceived(
            result
                .unwrap_or_else(|err| format!("{:?}", err)),
        )
    })
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchGood => {
            orders.skip();
            orders.perform_cmd(fetch_content("public/good.txt"));
        }
        Msg::FetchBad => {
            orders.skip();
            orders.perform_cmd(fetch_content("public/bad.txt"));
        }
        Msg::ContentReceived(content) => model.content = content
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::FetchGood), "fetch good"],
        button![ev(Ev::Click, |_| Msg::FetchBad), "fetch bad"],
        div![&model.content],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
