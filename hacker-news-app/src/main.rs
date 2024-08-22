use dioxus::prelude::*;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    let title = "タイトル";
    let by = "ばーい。";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! { div { "{title} by {by} ({score}) {time} {comments}"}}
}
