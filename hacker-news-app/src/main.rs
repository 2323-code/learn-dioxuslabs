use dioxus::prelude::*;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! { StoryListing {} }
}

fn StoryListing() -> Element {
    let title = "タイトル";
    let by = "ぴよぴよ";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! {
        div { padding: " 1.5rem ", position: " relative ",
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}
