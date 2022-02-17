import "../styles/globals.css";
import type { AppProps } from "next/app";
import Head from "next/head";
import Header from "../components/header";
import styles from "../styles/Home.module.css";
import "../styles/prism-gruvbox-light.css";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>{"impl Future { }"}</title>
        <link rel="icon" href="/favicon.ico" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin={"true"}
        />
        <link
          href="https://fonts.googleapis.com/css2?family=Major+Mono+Display&family=Raleway:ital,wght@0,500;0,600;0,700;0,800;1,500&display=swap"
          rel="stylesheet"
        />
      </Head>

      <main className={styles.main + " font-body"}>
        <Header></Header>
        <Component {...pageProps} />
      </main>
    </>
  );
}

export default MyApp;
