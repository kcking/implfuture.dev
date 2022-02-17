import { MDXProvider } from "@mdx-js/react";
import { Components } from "@mdx-js/react/lib";

export const components: Components = {
  h1: ({ children }) => <h1 className="text-4xl py-4">{children}</h1>,
  pre: ({ children }) => (
    <pre className="m-4 p-6 bg-gray-400 bg-opacity-10 rounded">{children}</pre>
  ),
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
