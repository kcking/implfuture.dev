import { readdir } from "fs/promises";
import Link from "next/link";

type Props = {
  blogs: Array<{ title: string; subtitle: string; path: string; date: string }>;
};

export default function Blog(props: Props) {
  return (
    <div>
      {props.blogs.map((blog) => (
        <>
          <Link href={blog.path}>
            <a className="text-inherit">
              <h1 className="font-display text-4xl">{blog.title}</h1>
              <div>{blog.subtitle}</div>
              <div className="text-xl">{blog.date}</div>
            </a>
          </Link>
        </>
      ))}
    </div>
  );
}

export const getStaticProps = async (): Promise<{ props: Props }> => {
  let blogs = await readdir("./pages/blog");
  const wow = await Promise.all(
    blogs
      .filter((blog) => blog.length > 0)
      .map(async (blog) => {
        const blogImport = await import("./blog/" + blog);
        blogImport.meta.title;
        return {
          title: blogImport.meta.title,
          path: "blog/" + blog.replace(/\.mdx$/, ""),
          date: blogImport.meta.date,
          subtitle: blogImport.meta.subtitle,
        };
      })
  );

  return { props: { blogs: wow } };
};
