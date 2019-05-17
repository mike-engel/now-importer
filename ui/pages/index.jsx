import React, { useState } from "react";
import styled, { createGlobalStyle } from "styled-components";
import Head from "next/head";
import { Heading, Text, fontFamily } from "../components/typography";
import { Button } from "../components/button";
import { lightGrey } from "../components/color";
import { Input } from "../components/form";
import { spacing } from "../utils/spacing";
import { parse, serialize } from "cookie";

const isServer = !process.browser;
const loginUrl = "https://zeit.co/oauth/authorize";
const storageKey = "now-auth-state";

const GlobalStyles = createGlobalStyle`
  *,
  *:before,
  *:after {
    box-sizing: border-box;

    &:focus:not(:focus-visible) { outline: none }
  }

  html,
  body {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
  }

  body {
    position: relative;
    font-size: 16px;
    font-family: ${fontFamily};
    font-style: normal;
    font-weight: 400;
    line-height: 1.4;
  }

  pre {
    background: ${lightGrey};
    padding: ${spacing(2)}px;
    white-space: pre-wrap;
  }

  @media(prefers-reduced-motion: reduce) {
    *,
    *:before,
    *:after {
      transition: none !important;
      animation: none !important;
    }
  }
`;

const randomString = () => {
  if (isServer) {
    const crypto = require("crypto");

    return crypto.randomBytes(20).toString("hex");
  }

  const array = new Uint32Array(20);

  window.crypto.getRandomValues(array);

  return [].map.call(array, byte => `0${byte.toString(16)}`.slice(-2)).join("");
};

const memoizeState = (serverCookie = "", setHeader) => {
  const state = randomString();
  const cookieConfig = serialize(storageKey, state, {
    sameSite: "lax",
    path: "/",
    secure: process.env.NODE_ENV === "production",
    maxAge: 600
  });
  const existingState = parse(isServer ? serverCookie : document.cookie);

  if (existingState[storageKey]) {
    return existingState[storageKey];
  }

  if (isServer) {
    console.log(setHeader.toString());
    setHeader("Set-Cookie", cookieConfig);
  } else {
    document.cookie = cookieConfig;
  }

  return state;
};

const Card = styled("div")`
  border: 1px solid ${lightGrey};
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  border-radius: ${spacing(0.5)}px;
  padding: ${spacing(2)}px;
  overflow: hidden;
`;

const Login = ({ oauthState }) => (
  <Card>
    <Text>Before you can import your existing website, you need to have a ZEIT account.</Text>
    <Text>Click the button below to login to ZEIT.</Text>
    <Button as="a" href={`${loginUrl}?client_id=${process.env.CLIENT_ID}&state=${oauthState}`}>
      Login
    </Button>
  </Card>
);

const Import = ({ onSubmit, disabled }) => {
  const [url, setUrl] = useState("");
  const proxyOnSubmit = evt => {
    evt.preventDefault();

    onSubmit(url);
  };

  return (
    <Card>
      <Text>
        Enter the full URL of your website below and hit the submit button to begin importing your
        website to ZEIT.
      </Text>
      <form onSubmit={proxyOnSubmit} disabled={disabled}>
        <Input
          label={{ value: "Website address" }}
          type="url"
          name="url"
          id="url"
          placeholder="http://yoursite.com"
          value={url}
          onChange={setUrl}
          disabled={disabled}
        />
        <Button disabled={disabled}>Begin import</Button>
      </form>
    </Card>
  );
};

const FeedbackMessage = ({ error, data }) => {
  if (!!error) {
    return (
      <>
        <Text>
          Uh oh, something went wrong importing your website. Here's what we got back from the
          server
        </Text>
        <pre>{error}</pre>
      </>
    );
  }

  if (!!data) {
    return (
      <Text>
        Success! Your website should now be available at <Link href={data}>{data}</Link>
      </Text>
    );
  }

  return null;
};

const handleImport = (setState, code) => async url => {
  setState({ loading: true });

  const data = {
    code,
    url,
    debug: true
  };

  try {
    const res = await fetch("/import", {
      method: "POST",
      body: JSON.stringify(data),
      headers: { "Content-Type": "application/json" }
    });
    const json = await res.json();

    if (!!json.error) throw new Error(json.error);

    setState({ loading: false, error: null, data: json.url });
  } catch (err) {
    console.error("Oops, there was an error importing that website: ", err);

    setState({ loading: false, error: err.message, data: null });
  }
};

const RawIndex = ({ className, code, state, cookie, setHeader }) => {
  const storedState = memoizeState(cookie, setHeader);
  const [{ loading, error, data }, setImportState] = useState({
    loading: false,
    error: null,
    data: null
  });

  if (!isServer && !!state && state !== storedState)
    throw new Error(
      "State values don't match. Something bad has happened. Please refresh and re-login."
    );

  return (
    <div className={className}>
      <GlobalStyles />
      <Head>
        <title>Now importer</title>
      </Head>
      <Heading>Now Importer</Heading>
      <Heading level={2} displayLevel={2}>
        Import your existing website onto the Now platform
      </Heading>
      <div aria-live="polite">
        <FeedbackMessage data={data} error={error} />
      </div>
      {!!code ? (
        <Import onSubmit={handleImport(setImportState, code)} disabled={loading} />
      ) : (
        <Login oauthState={storedState} />
      )}
    </div>
  );
};

RawIndex.getInitialProps = ({ query, req, res }) => ({
  cookie: req.headers.cookie,
  setHeader: res.setHeader.bind(res),
  code: query.code,
  state: query.state
});

const Index = styled(RawIndex)`
  width: 100%;
  max-width: 500px;
  margin: 10vh auto;
`;

export default Index;
