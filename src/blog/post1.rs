use super::*;

super::blog_style!();

pub fn post(md: &Metadata) -> Html {
    //  TODO: figure out how to embed dynamic content into mdx.
    //  could do this by parsing {} delimiters in text commonmark nodes
    mdx! {r#"
# Building a Blog like it's 2022 ✨

> This is a work-in-progress post about how I built this site. You can check out the source code on
> [GitHub](https://github.com/kcking/implfuture.dev).

What was I looking for in a blog? Three things. It should be easy to:

- draft blog posts in a familiar language (markdown)
- put everything into version control (git)
- most importantly, incorporate <em title="This string came from rust -> wasm!"><RustString /></em>
  components for interactive demos

Yes, that orange crab string actually comes from rust compiled to WebAssembly!
As another example, here's a <Counter /> backed by Rust that increments every
time you click it.

This setup will help illustrate the concepts I'll be covering in future posts,
such as an interactive [bevy](https://bevyengine.org) physics sim. I'll walk
through some of the highlights of getting this setup so you too can have a rusty
blog!

This post details some bits and pieces of how I got this setup working, but for the full version just head over to the [GitHub repo](https://github.com/kcking/implfuture.dev)!

If you've done your fair share of Next.js / MDX, you might want to just [skip to the rust part](#Rust) of the post right now :)

# Next.js + React + Typescript = 💖

While I'm excited about some of the [efforts](https://github.com/yewstack/yew)
going into [rust frontend](https://github.com/DioxusLabs/dioxus/) dev, I don't
think anything beats the productivity of React + Typescript just yet. Don't
worry, we'll get to the rust later!

The first step is to start with a Next.js Typescript project.

```bash
yarn create next-app --typescript
```

Next I recommend setting up [TailwindCSS](https://tailwindcss.com/docs/guides/nextjs). I have personally found it to be a joy
to work with, particularly when tweaking styling constantly while things are in
flux. Make sure you add `*.mdx` to the `pages` section of `tailwind.config.js`
so that classes that are only used in MDX aren't thrown away in a production
build.

And... that's pretty much it! Most of you have probably used Next.js before but
for those that are new, just run `yarn dev` and go to [http://localhost:3000](http://localhost:3000) to see
a live preview of your site.

# MDX

Speed of writing blog posts is very important to me. I also knew I wanted to
keep everything in source control and use a familiar syntax like Markdown. This
led me to pick MDX for writing actual blog posts.

The main components I needed were

- code blocks with syntax highlighting
- embeddable interactive react components
- an index page that lists each of the existing posts

With the following config changes, Next.js will compile any `.mdx` file as an independent page:

```js
//  next.config.js
const rehypePrism = require("@mapbox/rehype-prism");
const withMDX = require("@next/mdx")({
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [],
    rehypePlugins: [rehypePrism],
    providerImportSource: "@mdx-js/react",
  },
});
module.exports = withMDX({
  pageExtensions: ["js", "jsx", "ts", "tsx", "md", "mdx"],
  reactStrictMode: true,
};
```

I also added in `rehype-prism` for code syntax highlighting.

## Blog Index

No blog is complete without a page that lists all of the blog posts in
chronological order. I settled on a
[`blog.tsx`](https://github.com/kcking/implfuture.dev/blob/main/pages/blog.tsx)
page that uses `getStaticProps` at build time to iterate through all of the
`.mdx` files in the `blog` folder. Each post `export`s an object called `meta`
that is shaped like

```ts
type Meta = {
  date: string;
  title: string;
  subtitle: string;
  draft?: boolean;
};
```

The `import(..)` statement in `blog.tsx` compiles the mdx file into a javscript
module that exposes this `meta` object. We can then sort the posts by descending
date and format them into clickable rows just like we would for any other list
data rendered in React.

```tsx
return (
  <div>
    {props.blogs
      .sort((a, b) => (a.meta.date < b.meta.date ? 1 : -1))
      .filter(
        (blog) => !blog.meta.draft || process.env.NODE_ENV == "development"
      )
      .map((blog) => (
        <div key={blog.path}>... some bloggy dom elements ...</div>
      ))}
  </div>
);
```

This structure made it really simple to add the `draft` meta field to hide blog
posts that aren't ready for public consumption but that I want to see in my
local dev environment. _Yes I know they are visible in the public repo, think of it as learning in public 😁_

## Styling

MDX by default leaves all of the markdown elements unstyled. However, each type
of markdown element is rendered as the analagous HTML element. For example, `#`
maps to `h1`, `##` to `h2`, and `- ` to `li`. One way to style these elements is
by writing css for each of the corresponding dom elements. However, I am very
bought-in to using Tailwind. I also needed to set more than just styles, for
example prepending a `-` before each list item.

In addition to css, MDX lets you specify a React component for each dom element
rendered from the markdown. These dom element-to-component mappings are passed
into an `MDXProvider`, concentrating all of the blog styling and functionality
into the
[`bloglayout.tsx`](https://github.com/kcking/implfuture.dev/blob/main/components/bloglayout.tsx)
file.

This let me keep using Tailwind classes (yay!). It was also super easy to
implement deep-links to `h1` and `h2` elements by

- wrapping the header in a link with an `href` set to the header text
- setting the `id` of the header to that same text

```tsx
export const components: Components = {
  h1: ({ children }) => (
    <a href={`#${children?.toString()}`} className="text-inherit">
      <h1 className="text-4xl py-4" id={children?.toString()}>
        {children}
      </h1>
    </a>
  ),
  ...
};
```

_Yes my [XSS](https://owasp.org/www-community/attacks/xss/)-spidey-senses are tingling at the above, but hey we're in 2022 so as long as there is no `dangerouslySetInnerHTML` we're in the clear 😅_

# Rust

And now for the part we've all been waiting for... let's add some Rust! The
high-level idea is to create a rust library crate that is compiled to
WebAssembly using the `wasm-pack` CLI. That WebAssembly can then be imported
into a `.tsx` component or directly used in a `.mdx` file.

Our first step is to install
[`wasm-pack`](https://github.com/rustwasm/wasm-pack). There is an
[installer](https://github.com/rustwasm/wasm-pack), but I was able to compile
from source on windows using `cargo install wasm-pack` (after installing `perl`
and `openssl` with [`scoop`](https://scoop.sh/)).

## Creating the Rust Crate

`wasm-pack` makes the rest of the rust side of things pretty painless. Run
`wasm-pack new rust` to generate a complete cargo project in the `rust` folder.
There are just a couple of tweaks we should make to the generated project:

- since we want this folder to be part of the parent repository, remove the `.git` folder in the generated crate: <span className="inline-block"> `rm -rf rust/.git`</span>
- `rm` all of the CI stuff for now, like Travis and Appveyor
- Change the `edition` to `2021` in `Cargo.toml`
- Remove the `optional` field in the `wee_alloc` dependency in `Cargo.toml`
- Remove the `#[cfg(..)]` in `lib.rs` over the `ALLOC` line. We always want to use `wee_alloc` so we can minimize our binary size.

Great! Our wasm crate now exists. Run `wasm-pack` and make sure it builds
successfully. The build output should be in the `pkg` folder of your rust crate.

The default template calls `alert()` through javascript which will open a dialog
window (useful for debugging, but so mean to our poor blog readers!). Instead,
let's add a function to `lib.rs` to return a `String`.

```rust
//  lib.rs
#[wasm_bindgen]
pub fn rust_string() -> String {
    "rust 🦀".into()
}
```

> Adding `pub` and `#[wasm_bindgen]` will make this function callable from anything
> that imports this wasm module.

Note that we will have to run `wasm-pack build` every time we change any rust
code. I recommend using `cargo-watch` or some other tool to automatically
recompile the wasm whenever something changes. It's also a good idea to add
`wasm-pack build` to the `build` script in your `package.json` file.

## Calling from MDX

Using our wasm module is now as simple as importing it into an MDX file:

```tsx
//  blog_post.mdx
import { rust_string } from "../../rust/pkg";

a rust string {rust_string()}
```

That's it! We just need a few changes to our Next.js config to support wasm,
mainly a workaround for an issue with compiled wasm blobs being written to the
wrong directory:

```js
//  next.config.js
...
/** @type {import('next').NextConfig} */
module.exports = withMDX({
  pageExtensions: ["js", "jsx", "ts", "tsx", "md", "mdx"],
  reactStrictMode: true,
  webpack: function (config, options) {
    //  https://github.com/vercel/next.js/issues/29362#issuecomment-932767530
    config.output.webassemblyModuleFilename =
      options.isServer && !options.dev
        ? "../static/wasm/[id].wasm"
        : "static/wasm/[id].wasm";
    config.optimization.moduleIds = "named";
    config.experiments = { asyncWebAssembly: true, ...config.experiments };
    return config;
  },
});
```

And with that, we're ready to deploy our blog to production! 🚢 it!

## Custom Vercel Build

Up until now, everything we've done is independent of where you choose to deploy
your Next.js project. I personally chose to use [Vercel](https://vercel.com) as
it's the easiest way I know to deploy anything Next.js. While Vercel usually requires
zero configuration, to get rust + wasm working I had to make a few changes to
the build process.

Vercel recommends [installing
rust](https://vercel.com/docs/concepts/deployments/build-step#collapse-button-8)
using <span className="inline-block">`amazon-linux-extras install rust1`</span>.
However, this installs an older version of rust that doesn't support the [2021
edition](https://doc.rust-lang.org/edition-guide/rust-2021/index.html). I
installed rust using [`rustup`](https://rustup.rs) instead.

Next, the simplest way to install `wasm-pack` would be to compile it with `cargo install`. However this takes over a minute on the Vercel build vm. Instead, I
added a script to download the `wasm-pack` musl linux binary from GitHub.

Both of these steps are captured in a bash script:

```bash
# vercel-install.sh

#!/bin/bash
set -x
set -e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
export PATH="/vercel/.cargo/bin:$PATH"
curl "https://github.com/rustwasm/wasm-pack/releases/download/v0.10.2/wasm-pack-v0.10.2-x86_64-unknown-linux-musl.tar.gz" -o wasm-pack.tar.gz -s -L
tar xvf wasm-pack.tar.gz --wildcards --no-anchored 'wasm-pack' --strip-components=1
rm wasm-pack.tar.gz
chmod +x wasm-pack
mv wasm-pack /usr/bin
yarn install
```

To make Vercel run this bash script before your build, set the `INSTALL COMMAND`
in your Vercel deployment settings to `bash ./vercel-install.sh`. Finally, set
the `BUILD COMMAND` to `yarn build` so that `wasm-pack build` is run before
`next build`. I hope Vercel considers adding rust + wasm-pack to a pre-baked
build image to make this whole process a bit easier!

Thanks for reading! I hope this post helped smooth out a few of the rough edges
with getting rust integrated into your own blog. Feel free to clone, fork, or
pull request this site on [GitHub](https://github.com/kcking/implfuture.dev)!

"#    }
}
