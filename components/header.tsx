import Link from "next/link";

export default function Header() {
  return (
    <div className="flex justify-evenly flex-wrap w-full">
      <h1 className="font-display text-6xl p-10">
        <Link href="/" passHref>
          <button className="p-4">{"impl Future {}"}</button>
        </Link>
      </h1>
      <div className="flex">
        <Link href="/blog" passHref>
          <button className="p-4 text-3xl">Blog</button>
        </Link>
        <Link href="/projects" passHref>
          <button className="p-4 text-3xl">Projects</button>
        </Link>
        <Link href="https://twitter.com/4kevinking" passHref>
          <button className="p-4 text-3xl">Contact</button>
        </Link>
      </div>
    </div>
  );
}
