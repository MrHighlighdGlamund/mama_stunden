use dioxus::prelude::*;

use crate::View;
#[component]
pub fn choose_month(view: Signal<View>) -> Element {
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
