---
layout: ../../../layouts/DocsLayout.astro
title: "Running the Examples"
pubDate: 2024-01-20
author: "Reyth"
---

To best get to know the capabilities of the framework, we recommend to start with running the example canisters that have been included in the Pluto repository. These examples demonstrate the current capabilities of Pluto as well as the basic usage of most important features, and can be a great start for your own project.

## Core features of Pluto

First and foremost, it's important to clarify what Pluto can allow you to do. Pluto is a framework that, at its core, is designed to work in two scenarios:

- Creating a RESTful API hosted on-chain
- Building a server(blockchain)-side rendered web app with an MVC architectural pattern.

Both use cases are documented with basic examples, and more advanced demos will come with time.

## Cloning the repository

To get access to the examples, you first will need to clone the pluto repository. You can use the following command:

```sh
git clone https://github.com/pu0238/pluto.git
```

With the pluto source code available locally, you can navigate to the catalog and then open the examples

```sh
cd pluto/examples
```

In that folder you will see the `dfx.json` file, which can be thought of as a 'root' of a dfx workspace. This file stores information about the containers defined in the subfolders and instructs the command line tool how to deploy them to a node.

In one terminal you will need to open the local node for Internet Computer. This will allow you to deploy and test the canisters locally. In your terminal window, make sure that you are currently in the `examples` directory and type the following:

```sh
dfx start
```

This will open the node. Keep that terminal window open in the background.

The next step is deploying the canisters to the local network. In a new terminal window you will need to type in the following command:

```sh
dfx deploy
```

This command will trigger the build of your canisters for WASM and then attempt to deploy the built binaries to the network. This might take some time. Once its done, you should see the confirmation of your deployment, with the addresses of your canisters printed to the terminal.

```
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    http_server: http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=br5f7-7uaaa-aaaaa-qaaca-cai
    http_server_body_validation: http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=be2us-64aaa-aaaaa-qaabq-cai
    http_server_cors: http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=bd3sg-teaaa-aaaaa-qaaba-cai
    http_server_html_template: http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
```

## Accessing the canisters through HTTP

Congratulations, the set up is done. Now we just need to open our webpages in the browser window. The terminal above gives us URLs that we could click, but they will lead to an HTTP interface of a default IC-provided canister (Candid), where you can remotely call functions of other contracts deployed on the network. This is not what we're after here. We want to open our canister in the same way that Candid is linked there in the terminal.

To do this we have to look at our containers - let's take the `http_server` canister for example:

```
http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=br5f7-7uaaa-aaaaa-qaaca-cai
```

In that URL we have the loopback IP and two parameters. One of them (canisterId) points to Candid, and then the `id` parameter is candid's qury param that specified which canisdter we want to interface with through candid. So logically, what we can do to access our newly deployed `http_server` is to replace the Candid address with our own and remove the parameter, like so:

```
http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai
```

While this works in our simple example, this is not the ideal way of accessing our canisters, as we need to specify its address through a query param. The recommended way is to change the loopback address to `localhost`, and put the canister address before it. That way we end up with no unnecessary query params that could break the website:

```
http://br5f7-7uaaa-aaaaa-qaaca-cai.localhost:4943/
```

This transfers us to the same webpage, but it is much cleanber and doesn't cause any errors. This is also how you refer to containers on the actual Internet Computer public node.
