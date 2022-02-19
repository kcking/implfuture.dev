import "../styles/globals.css";
import type { AppProps } from "next/app";
import Head from "next/head";
import Header from "../components/header";
import styles from "../styles/Home.module.css";
import "../styles/prism-vs.css";
import "../styles/prism-vsc-dark-plus.css";
import PlausibleProvider from "next-plausible";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>{"impl Future { }"}</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <PlausibleProvider domain={"implfuture.dev"}>
        <main className={styles.main + " font-body"}>
          <Header></Header>
          <Component {...pageProps} />
        </main>
      </PlausibleProvider>
    </>
  );
}

export default MyApp;
