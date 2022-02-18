import { readdir } from "fs/promises";
import Link from "next/link";

type Props = {
  blogs: Array<{
    path: string;
    meta: {
      date: string;
      draft?: boolean;
      title: string;
      subtitle: string;
    };
  }>;
};

export default function Blog(props: Props) {
  return (
    <div>
      {props.blogs
        .sort((a, b) => (a.meta.date < b.meta.date ? 1 : -1))
        .filter(
          (blog) => !blog.meta.draft || process.env.NODE_ENV == "development"
        )
        .map((blog) => (
          <div key={blog.path}>
            <Link href={blog.path}>
              <a className="text-inherit">
                <h1 className="font-display text-4xl">{blog.meta.title}</h1>
                <div>{blog.meta.subtitle}</div>
                <div className="text-xl">
                  {new Date(blog.meta.date).toLocaleString("default", {
                    month: "short",
                    day: "numeric",
                    year: "numeric",
                  })}
                </div>
              </a>
            </Link>
          </div>
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
          path: "blog/" + blog.replace(/\.mdx$/, ""),
          meta: blogImport.meta,
        };
      })
  );

  return { props: { blogs: wow } };
};
