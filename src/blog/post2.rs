use super::*;

pub fn post_2(_: &Metadata) -> Html {
    //  split post into multiple parts to workaround `locals exceed maximum` error
    html! {
        <>
        {post_2_0()}
        {post_2_1()}
        </>
    }
}

fn post_2_0() -> Html {
    include_mdx!("src/blog/post2_0.mdx")
}
fn post_2_1() -> Html {
    include_mdx!("src/blog/post2_1.mdx")
}
