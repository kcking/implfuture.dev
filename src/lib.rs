mod blog;
mod projects;

use std::collections::HashMap;

use projects::Projects;
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    BlogIndex,
    #[at("/blog/:slug")]
    BlogPost { slug: String },
    #[at("/projects")]
    Projects,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}

#[function_component]
fn Navbar() -> Html {
    html! {
        <div class="flex justify-evenly flex-wrap w-full navbar">
            <h1 class="font-display text-6xl p-10">
                <Link<Route> to={Route::Home}><div class="p-4">{"impl Future {}"}</div></Link<Route>>
            </h1>
            <div class="flex items-center">
                <Link<Route> classes="p-4 text-3xl" to={Route::BlogIndex}>
                    <button >
                        {"Blog"}
                    </button>
                </Link<Route>>
                <Link<Route> classes="p-4 text-3xl" to={Route::Projects}>
                    <button >
                        {"Projects"}
                    </button>
                </Link<Route>>
                <a class="p-4 text-3xl" href="https://twitter.com/4kevinking">{"Contact"}</a>
            </div>
        </div>
    }
}

#[function_component]
fn Home() -> Html {
    html! {}
}

fn switch(route: Route) -> Html {
    html! {
        <main class="font-body flex items-center px-2">
            <Navbar />
            {
                match route {
                    Route::Home => html! {
                        <>
                        <div class="w-full font-body text-3xl my-10 flex justify-evenly px-2 flex-wrap">
                            <div class="max-w-md">
                                <div>
                                    <strong class="font-extrabold">{"im·pl"}</strong>
                                </div>
                                <div>{"[im-pʊl]"}</div>
                                <div class="">
                                    <em>{"v"}</em>{". to bring into existence something previously impossible,
                                    usually by means of code"}
                                </div>
                            </div>
                            <div></div>
                            <div></div>
                        </div>
                        <div class="w-full font-body text-3xl my-10 flex justify-evenly px-2 flex-wrap">
                            <div class="w-full text-xl max-w-sm">
                                {"Hey there, I'm Kevin! I like rust, kubernetes, virtual reality, bevy,
                            audio synthesis, and many other things. I have built lots of infrastructure, from big data
                            analytics to realtime video processing. In my spare time I am branching out into
                            3D working with the bevy game engine."}
                            </div>
                            <div></div>
                            <div></div>
                        </div>
                        </>
                    },
                    Route::BlogIndex => html! {
                        <div class="w-full font-body flex justify-evenly px-2 flex-wrap">
                            {blog::blog_index()}
                        </div>
                    },
                    Route::BlogPost{slug} => {
                        blog::render(&slug)
                    }
                    Route::Projects => html! {
                        <div class="w-full flex justify-evenly px-2 flex-wrap">
                            <Projects />
                        </div>
                    },
                }
            }
        </main>
    }
}
