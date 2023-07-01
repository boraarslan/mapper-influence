import Head from "next/head";
import type { AppProps } from "next/app";
import { CookiesProvider } from "react-cookie";
import {
  QueryClient,
  QueryClientProvider,
  Hydrate,
} from "@tanstack/react-query";
import Layout from "@components/Layout";

import "../styles/globals.scss";

const queryClient = new QueryClient();

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <meta
          name="description"
          content="Track and share your osu! mapping influences."
        />
        <meta
          name="keywords"
          content="osu, mapping, map, beatmap, beatmaps, community, influence, style"
        />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1.0"></meta>
        <meta name="author" content="Fursum"></meta>
        <link rel="icon" href="/svg/Influences.svg" />
        <link rel="shortcut icon" href="/svg/Influences.svg" />
        <link rel="mask-icon" href="/svg/Influences.svg" color="#000000" />
        <title>Mapper Influences</title>
      </Head>

      <QueryClientProvider client={queryClient}>
        <Hydrate state={pageProps.dehydratedState}>
          <CookiesProvider>
            <Layout>
              <Component {...pageProps} />
            </Layout>
          </CookiesProvider>
        </Hydrate>
      </QueryClientProvider>
    </>
  );
}

export default App;
