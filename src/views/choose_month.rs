use dioxus::prelude::*;

use crate::View;
#[component]
pub fn choose_month(view: Signal<View>) -> Element {
    rsx! {
        // create google link
        a { style: "display: flex; align-items: center; justify-content: space-between; width: 95vw; margin: 6px; padding: 8px; border: 2px solid #E0A0B0; background-color: #A3B8B6; color: white;",
            href: format!("https://www.google.de"),
            "Link"
        }
    }
}
