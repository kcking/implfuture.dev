mod post1;
mod post2;
mod syntaxhighlight;

use log::info;
use time::macros::date;
use yew::{
    function_component, html, include_mdx, mdx, mdx_style, use_state, Children, Html, Properties,
};
use yew_router::prelude::Link;

use crate::{blog::syntaxhighlight::HighlightCode, Route};

macro_rules! blog_style {
    () => {
        mdx_style!(
            h1: MyH1,
            h2: MyH2,
            blockquote: MyBlockquote,
            pre: HighlightCode,
            p: MyP,
            li: MyLi,
            ul: MyUl,
        );
    };
}
pub(crate) use blog_style;

#[derive(PartialEq, Properties)]
pub struct ChildProps {
    #[prop_or_default]
    children: Children,
}
#[function_component]
fn MyH1(c: &ChildProps) -> Html {
    let mut tag = String::new();
    for c in c.children.iter() {
        match c {
            yew::virtual_dom::VNode::VText(t) => {
                tag += &t.text.to_string();
            }
            _ => (),
        };
    }
    tag = tag.replace(" ", "-").to_lowercase();
    tag.truncate(12);
    html! {
      <a class="text-inherit" href={format!("#{tag}")}>
        <h1 id={tag} class="text-4xl py-4">
          {c.children.clone()}
        </h1>
      </a>
    }
}

#[function_component]
fn MyH2(c: &ChildProps) -> Html {
    let mut tag = String::new();
    for c in c.children.iter() {
        match c {
            yew::virtual_dom::VNode::VText(t) => {
                tag += &t.text.to_string();
            }
            _ => (),
        };
    }
    tag = tag.replace(" ", "-").to_lowercase();
    tag.truncate(12);
    html! {
      <a class="text-inherit" href={format!("#{tag}")}>
        <h2 id={tag} class="text-2xl py-4">
          {c.children.clone()}
        </h2>
      </a>
    }
}

#[function_component]
fn MyPre(c: &ChildProps) -> Html {
    html! {
      <pre class="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
      <HighlightCode>{c.children.clone()}</HighlightCode>
      </pre>
    }
}

#[function_component]
fn MyBlockquote(c: &ChildProps) -> Html {
    html! {
      <blockquote class="text-black/70 dark:text-white/50 border-l-8 px-2 my-2 italic">
        {c.children.clone()}
      </blockquote>
    }
}

#[function_component]
fn MyP(c: &ChildProps) -> Html {
    html! {
      <p class="py-2 text-lg">
        {c.children.clone()}
      </p>
    }
}

//  ul: ({ children }) => <div className="px-4">{children}</div>,
// li: ({ children }) => <p className="py-1"> - {children}</p>,

#[function_component]
fn MyUl(c: &ChildProps) -> Html {
    html! {
      <div class="px-4">{c.children.clone()}</div>
    }
}

#[function_component]
fn MyLi(c: &ChildProps) -> Html {
    html! {
      <p class="py-1">{" - "}{c.children.clone()}</p>
    }
}

#[derive(Properties, PartialEq)]
struct HeyProps {
    name: String,
}
#[function_component]
fn HeyComponent(props: &HeyProps) -> Html {
    let count = use_state(|| 0);
    let s = count.to_string();

    html! {
        <>
        <p>{"said hey to "}{&props.name}{" "} <em onclick={ move |_| count.set(*count + 1)}>{s}</em>
        {" times"}</p>
        </>
    }
}

#[function_component]
fn Counter() -> Html {
    let count = use_state(|| 0);
    let s = count.to_string();

    html! {
        <button onclick={ move |_| count.set(*count + 1)} class="bg-gray-300/30 rounded select-none p-2">
        {"Counter "}{s}
        </button>
    }
}

#[function_component]
fn RustString() -> Html {
    html! {
      <span class="text-orange-600">{"rust 🦀"}</span>
    }
}

pub struct Metadata {
    title: &'static str,
    date: time::Date,
    slug: &'static str,
    subtitle: &'static str,
}

const BLOG_POSTS: &[(Metadata, &dyn Fn(&Metadata) -> Html)] = &[
    (
        Metadata {
            date: date!(2022 - 2 - 15),
            slug: "building-a-blog-like-its-2022",
            title: "Building a Blog Like it's 2022 ✨",
            subtitle: "With Next.js, typescript, react, mdx, rust + wasm",
        },
        &post1::post,
    ),
    (
        Metadata {
            date: date!(2022 - 9 - 3),
            slug: "rewriting-modern-web-in-rust",
            title: "Rewriting Modern Web in Rust",
            subtitle: "ssr, mdx, hooks",
        },
        &post2::post_2,
    ),
];

pub fn render(slug: &str) -> Html {
    let post_content = BLOG_POSTS
        .iter()
        .find(|(meta, _)| &slug == &meta.slug)
        .map(|(meta, post)| post(meta))
        .unwrap_or(mdx! {r#"Post not found :("#});
    html! {
      <div class="w-full md:max-w-4xl p-2">
        {post_content}
      </div>
    }
}

pub fn blog_index() -> Html {
    let fmt = time::macros::format_description!("[month repr:short] [day], [year]");
    BLOG_POSTS
        .iter()
        .map(|(metadata, _)| {
            html! {
              <div class="py-4">
                <Link<Route> classes="text-inherit" to={Route::BlogPost { slug: metadata.slug.into() }}>
                  <h1 class="text-4xl font-display">
                    {&metadata.title}
                  </h1>
                  <div> {&metadata.subtitle} </div>
                  <div class="text-xl"> {&metadata.date.clone().format(&fmt).unwrap_or_default()} </div>
                </Link<Route>>
              </div>
            }
        })
        .collect()
}
