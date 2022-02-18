import { MDXProvider } from "@mdx-js/react";
import { Components } from "@mdx-js/react/lib";

export const components: Components = {
  h1: ({ children }) => (
    <a href={`#${children?.toString()}`} className="text-inherit">
      <h1 className="text-4xl py-4" id={children?.toString()}>
        {children}
      </h1>
    </a>
  ),
  h2: ({ children }) => (
    <a href={`#${children?.toString()}`} className="text-inherit">
      <h2 className="text-2xl py-4" id={children?.toString()}>
        {children}
      </h2>
    </a>
  ),
  pre: ({ children }) => (
    <pre className="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
      {children}
    </pre>
  ),
  blockquote: ({ children }) => (
    <blockquote className="text-black/70 border-l-8 px-2 my-2 italic">
      {children}
    </blockquote>
  ),
  p: ({ children }) => <p className="py-2 text-lg">{children}</p>,
  ul: ({ children }) => <div className="px-4">{children}</div>,
  li: ({ children }) => <p className="py-1"> - {children}</p>,
};

//@ts-ignore
export function Layout({ children }) {
  return (
    <>
      <MDXProvider components={components as any}>
        <div className="max-w-4xl w-full p-2">{children}</div>
      </MDXProvider>
    </>
  );
}
