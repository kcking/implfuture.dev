use wasm_bindgen::prelude::*;
use yew::prelude::*;

fn main() {
    println!("hi!");
    yew::Renderer::<gaia::App>::new().hydrate();
}
