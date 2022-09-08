use web_sys::{console, Element};
use yew::{
    prelude::*,
    virtual_dom::{VNode, VTag},
};

#[function_component]
pub fn HighlightCode(c: &super::ChildProps) -> Html {
    //  IDEA: try NodeRef and call Prism.highlightElement on it
    //      but how do we store a ref??
    let code_ref = use_state_eq(|| NodeRef::default());
    let mut code_tag = c.children.iter().next().unwrap().clone();
    match &mut code_tag {
        VNode::VTag(t) => t.node_ref = (*code_ref).clone(),
        _ => {}
    };

    use_effect_with_deps(
        move |_| {
            console::log_1(&"highlighting...".to_string().into());
            let element = code_ref.cast::<Element>().unwrap();
            prism::highlightElement(element.clone());
            move || {
                element
                    .closest(".codecontainer")
                    .ok()
                    .flatten()
                    .map(|e| e.remove());
            }
        },
        c.children.clone(),
    );

    html! {
        <div class="codecontainer">
            <pre class="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
                {code_tag}
            </pre>
        </div>
    }
}

mod prism {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Language;
        #[wasm_bindgen(js_namespace = Prism)]
        pub static languages: Language;

        #[wasm_bindgen(method, structural, indexing_getter)]
        pub fn get(this: &Language, prop: String) -> Language;

        #[wasm_bindgen(js_namespace = Prism)]
        pub fn highlight(code: String, lang: Language) -> String;

        #[wasm_bindgen(js_namespace = Prism)]
        pub fn highlightElement(element: web_sys::Element);
    }
}
