import { MDXProvider } from "@mdx-js/react";
import { Components } from "@mdx-js/react/lib";

export const components: Components = {
  h1: ({ children }) => <h1 className="text-4xl py-4">{children}</h1>,
  pre: ({ children }) => (
    <pre className="m-4 p-6 bg-gray-300/5 rounded ">{children}</pre>
  ),
  blockquote: ({ children }) => (
    <blockquote className="text-black/70 border-l-8 px-2 my-2 italic">
      {children}
    </blockquote>
  ),
  p: ({ children }) => <p className="py-2 text-lg">{children}</p>,
};

//@ts-ignore
export function Layout({ children }) {
  return (
    <>
      <MDXProvider components={components as any}>
        <div className="max-w-4xl w-full">{children}</div>
      </MDXProvider>
    </>
  );
}
