use std::sync::Arc;

use stylist::{css, style, YieldStyle};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
struct ProjectProps {
    description: Html,
    name: String,
    image: String,
}

#[function_component]
fn Project(props: &ProjectProps) -> Html {
    let img_style = css!(
        r#"
    background-image: ${image};
    background-size: 80% 80%;
    background-position: 50% 20%;
    background-repeat: no-repeat;
    "#,
        image = &props.image
    );
    html! {
    <div
      class="group p-2 border-0 border-zinc-700 border-solid w-64 h-64 flex flex-col justify-between select-none "
      tabIndex={0}
      key={props.name.as_str()}
    >
      <div
        class={
          classes!("w-60 h-60 absolute group-hover:invisible
          group-focus:invisible".split_ascii_whitespace().collect::<Vec<_>>(),
          img_style)
        }
      ></div>
      <div class="invisible group-hover:visible group-focus:visible">
      {
        props.description.clone()
      }
      </div>
      <div class="text-2xl font-display place-self-center">
        {&props.name}
      </div>
    </div>
    }
}

#[function_component]
pub fn Projects() -> Html {
    html! {
        <div class="flex flex-wrap gap-10 justify-center max-w-6xl">
        <Project name="Bevy OpenXR" image="url(/img/bevy.svg)" description={html!{
            <>
            {"Unity and Unreal Engine are practically the only options when
            developing VR games and experiences. I am actively bringing OpenXR
            support to the "}<a href="https://bevyengine.org">{"Bevy game
            engine"}</a>{" in order to spread the benefits of rust to XR
            development. "}
            </>
        }} />

        <Project name="Ovrlay" image="url(/img/ovrlay.svg)" description={html!{
            <>
                {"View Discord notifications while you're in VR. Built with OpenVR
                rust bindings and "}
                <a href="https://github.com/iced-rs/iced">
                <code>{"iced"}</code>{" "}
                </a>{"native GUI toolkit. Forked "}<code>{"iced_glow"}</code>{" to render to an OpenGL
                texture which is then composited into VR using the OpenVR overlay API."}
                <a href="https://store.steampowered.com/app/1384020/Ovrlay__VR_Discord_Notifications/">
                {"Published on Steam."}
                </a>
            </>
        }} />

        <Project name="Bigroom" image="url(/img/bigroom.svg)" description={html!{
            <>
            {"Social video chat for large groups. Built with WebRTC and WebAudio,
        works directly in the browser. Full stack Rust web application using "}
        <code>{"async-tungstenite"}</code>{" and "}<code>{"yew"}</code>{"."}
            </>
        }} />

        <Project name="Just Fly" image="url(/img/justfly.png)" description={html!{
            <>
                {"A mod for the factory simulation game "}
                <a href="https://www.satisfactorygame.com/">{"Satisfactory"}</a>{". Provides
                ergonomic flight controls for an optimal \"Creative Mode\"
                experience. Built using Unreal Engine blueprints. Published on "}
                <a href="https://ficsit.app/mod/JustFly">{"ficsit.app"}</a>{" and available on "}
                <a href="https://github.com/kcking/justfly">{"GitHub"}</a>{"."}
            </>
        }} />

        <Project name="Clickhouse" image="url(/img/clickhouse.svg)" description={html!{
            <>
            {"Rearchitected "}<a href="https://mux.com">{"Mux"}</a>
            {"'s video analytics product from Citus to "}
            <a href="https://clickhouse.com">{"ClickHouse"}</a>{". Deployed using
            Kubernetes to be horizontally and vertically scalable with zero
            downtime. Check out the corresponding "}
            <a
              href="https://mux.com/blog/from-russia-with-love-how-clickhouse-saved-our-data/"
              rel="noopener noreferrer"
            >
              {"blog post"}
            </a>
            {"."}
            </>
        }} />

        <Project name="Krypton" image="url(/img/krypton.svg)" description={html!{
            <>
            <a href="https://krypt.co" rel="noopener noreferrer">
            {"Phone-based phishing-proof 2FA."}
            </a>{" Built on a trustless infrastructure using end-to-end encryption between
            user's devices. Technology aquired by Akamai. All client software
            published on "}<a href="https://github.com/kryptco/">{"GitHub"}</a>{"."}
            </>
        }} />

        <Project name="Ears" image="url(/img/chrome.svg)" description={html!{
            <>
            <a
            href="https://chrome.google.com/webstore/detail/ears-bass-boost-eq-any-au/nfdfiepdkbnoanddpianalelglmfooik"
            rel="noopener noreferrer"
          >
            {"Chrome extension"}
          </a>{" that provides a graphical equalizer, volume, or bass boost for any
          webpage. Used by more than 300,000 audiophiles, hard-of-hearing, and
          transcriptionists world-wide."}
            </>
        }} />

        </div>
    }
}
