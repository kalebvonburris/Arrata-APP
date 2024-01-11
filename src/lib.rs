pub mod character;
pub mod dice;

use character::render::*;
use character::structs::*;

use dioxus::prelude::*;

/// The main application.
pub fn app(cx: Scope) -> Element {
    let character = use_ref(cx, Character::new);
    let render_dice_roll = use_state(cx, || false);

    let arrata_style = r#"
    body { background-color: black; color: white; }
    input { background-color: black; color: white; }
    select { background-color: black; color: white; }
    "#;

    cx.render(rsx! {
        style { arrata_style }

        div { class: "px-5 py-5 origin-center justify-center self-center items-center content-center flex",
            // Arrata logo
            img {
                class: "w-24 h-24 md:w-28 md:h-auto md:rounded-none rounded-full mr-10",
                src: "public/rat.png",
                alt: "",
                width: 300,
                height: 300
            }

            h1 { class: "text-center text-9xl font-mono font-extrabold", "ARRATA" }
        }

        br {}

        div { class: "px-5 py-5 origin-center justify-center self-center items-center content-center flex space-x-3",
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| character.read().write_to_file().unwrap(),
                "Save Character"
            }
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| character.set(Character::from_file().unwrap()),
                "Load Character"
            }
        }

        br {}

        render_character { character: character }

        if **render_dice_roll {
            render! { "stuff" }
        }

        button {
            onclick: |_| {
                let new_state = !**render_dice_roll;
                render_dice_roll.set(new_state);
            },
            "render stuff"
        }
    })
}
