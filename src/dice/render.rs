use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsX, Icon};

use crate::{
    character::structs::*,
    dice::{roll::roll_stat, structs::*},
};

#[component(no_case_check)]
pub fn render_rolls<'a>(cx: Scope, state: &'a UseState<(bool, Option<Stat>)>) -> Element {
    // Stat being passed must be given as a Some(Stat) otherwise the app will crash.
    let stat = state.get().1.clone().unwrap();

    let dice_results: &UseState<Option<DiceResult>> = use_state(cx, || None);

    cx.render(rsx! {
        div { class: "z-10 fixed flex justify-center content-center max-w-[80%] w-96 h-fit border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 px-2 py-2",
                div {
                    class: "bg-slate-900 hover:bg-slate-600",
                    onclick: move |_| {
                        state
                            .with_mut(|state| {
                                state.0 = false;
                                state.1 = None;
                            });
                    },
                    Icon { width: 50, height: 50, fill: "red", icon: BsX }
                }
            }
            div {
                // Stat
                div { class: "flex flex-wrap px-2 py-2 justify-center content-center",
                    // Stat Name
                    h2 { class: "text-xl text-center font-mono",
                        "{stat.name}"
                    }

                    br {}

                    // Quality + Quantity
                    div { class: "inline-flex",
                        div { class: "text-xl font-mono",
                            "{stat.quality}"
                        }
                        div { class: "text-xl font-mono",
                            "{stat.quantity}"
                        }
                    }

                }
                // Rolling
                div { class: "flex w-full content-center",
                    button {
                        class: "max-w-fit font-mono place-self-center bg-slate-900 hover:bg-slate-600 py-2 px-2 rounded",
                        onclick: move |_| {
                            dice_results
                                .with_mut(|results| {
                                    *results = Some(roll_stat(&stat));
                                });
                        },
                        "Roll!"
                    }
                }


                if let Some(results) = dice_results.get() {
                    rsx!( div { class: "font-mono", "{results:#?}" } )
                }
            }
        }
    })
}
