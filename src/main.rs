use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
                div {
                    style: "display: flex; flex-direction: column; height: 100vh; margin: 0; padding: 0;",

                    div {
                            style: "display: flex; flex: 0 0 10%; width: 100%;",

                            button {
                                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                                onclick: move |_| {
                                    println!("Monat auswaehlen");
                                },
                                "Monat auswaehlen"
                            }

                            button {
                                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                                onclick: move |_| {
                                    println!("Button 2 clicked!");
                                },
                                "Neuer Monat"
                            }
                        }

                    // Scrollable area - 80% height
                    div {
                            id: "scrollable", // ✅ This matches the CSS selector
                            div {
                                button {
                                    "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                                }
                                button {
                                    "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                                }
            button {
                                    "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                                }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
        button {
                                "01.10.2024  8:00 - 10:00  Linie 76/77   08.50 Std  145.00 Std"
                            }
                            }

                        }

                    // Bottom button - 10% height
                    div {
                        style: "display: flex; flex: 0 0 10%; width: 100%;",
                     button {
                        onclick: move |_| {
                            println!("+++ tag hinzufuegen +++");
                        },
                        "+++ Neuer Eintrag +++"
                    }

    button {
                        onclick: move |_| {
                            println!("+++ Neuer Eintrag +++");
                        },
                        "Teilen"
                    }
                    }

                }
                }
}
