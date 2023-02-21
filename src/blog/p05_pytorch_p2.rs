use super::*;

super::blog_style!();

pub fn post_5(_: &Metadata) -> Html {
    include_mdx!("src/blog/p05_pytorch_p2.mdx")
}
