use super::*;

super::blog_style!();

pub fn post_4(_: &Metadata) -> Html {
    include_mdx!("src/blog/p04_pytorch.mdx")
}
