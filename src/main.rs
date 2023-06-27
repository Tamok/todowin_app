use dioxus::prelude::*;

fn main() {
    dioxus::start::<App>();
}

struct App;

impl Component for App {
    fn render(&self, _cx: &mut Context) -> Element {
        // The app's main UI goes here
    }
}