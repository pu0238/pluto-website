---
layout: ../../../layouts/DocsLayout.astro
pubDate: 2024-01-25
author: "Pu0238"
---

# Getting Started

In this section we're going to go over the steps you need to take to start using Pluto in your own projects. It's going to go a bit more in-depth than the previous section, but without getting into any advanced concepts just yet.

In this section we will cover the following topics:

- The installation process of all the necessary tools
- Your first REST API project using the example code
- Features overview of the Pluto tooling

## Tools and Requirements

Pluto is a Rust-based framework, so it will require the Rust programming language, as well as its dependencies. Furthermore, Internet Computer-specific tooling has to be installed. This is what we will be covering in this section.

### Rust

To download and install Rust, as well as cargo, you will need to follow the instructions from the [official website' installation page](https://www.rust-lang.org/tools/install). Once you're done, you can verify the installation process was successful by running the `cargo --version` command. The expected output of the command is something similar to the version tag below:

```
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

### Internet Computer tooling

To install the IC SDK and the `dfx` command line tool, please follow the instructions from the [official ICP docs](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/). Once that is finished, you should be able to run `dfx --version` with an output similar to the version tag shown below:

```
dfx 0.15.3
```

### Other requirements

To follow the tutorial in the next section you will also need the `git` binary, as well as some kind of text editor, like [Visual Studio Code](https://code.visualstudio.com/).

