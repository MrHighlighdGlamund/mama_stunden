use dioxus::prelude::*;
pub mod database;

#[derive(Clone, PartialEq)]
pub enum View {
    ProfileSelection,
    ChooseMonth,
    CreateMonth,
    Entrys,
    CreateEntry,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let view = use_signal(|| View::ProfileSelection);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        match *view.read() {
            View::ProfileSelection => rsx! { ProfileSelection { view } },
            _ => rsx! { ChooseMonth { view } },
        }
    }
}
#[component]
pub fn ChooseMonth(view: Signal<View>) -> Element {
    rsx! {
         button {
                                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                                onclick: move |_| {
                                    view.set(View::ProfileSelection);
                                },
                                "Monat auswaehlen"
                            }
    }
}
#[component]
pub fn main_view (view: Signal<View>) -> Element {
    rsx! {
         button {
                                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                                onclick: move |_| {
                                    view.set(View::ProfileSelection);
                                },
                                "Monat auswaehlen"
                            }
    }
}

pub fn ProfileSelection(view: Signal<View>) -> Element {
    let profiles = database::get_all_profiles();
    rsx! {
        for profile in profiles {
            div {
            button {
                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                onclick: move |_| {
                    view.set(View::ChooseMonth);
                },
                "{profile.name}"
            }
            }
        }

                            
    }
}
