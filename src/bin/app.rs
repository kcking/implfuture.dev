fn main() {
    println!("hi!");
    yew::Renderer::<implfuture::App>::new().hydrate();
}
