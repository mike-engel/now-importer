# now-importer

> Easily import your static websites into ZEIT's [now](https://now.sh) platform.

This is a small project to import and deploy an existing static website into now.sh. There are three parts to this project

1. A CLI to import an existing website
2. A now lambda to do the work of downloading, configuring, uploading, and deploying the website
3. A [UIHook](https://zeit.co/docs/integrations/) to allow others to install this onto their ZEIT account and have a UI to import their sites

## Using

As of right now, the binary isn't pre-built for download. I may do that one day.

To use the integration, head over to the [integration page](https://zeit.co/integrations/now-importer).

## Installing

After downloading the projct, you have a couple of paths forward, depenidng on what you want to run and/or contribute to.

### Binary and/or Lambda

For these parts, you need `rust` and `cargo`. The easiest way is through [rustup](https://rustup.rs). After you have that installed, you can compile the project or check for any errors and warnings. The rust parts of this repo use [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

```sh
# compile everything
cargo build

# check for errors and warnings
cargo check

# run the binary
cargo run -p now_importer_binary -- --help
```

### UI Hook

For this part, you will need `node`, `npm`, and `now`. There are many ways to install `node` and `npm`. A quick search on the internet should help you install them. To get `now`, download them from ZEIT's [website](https://zeit.co/download).

For testing the UIHook, you'll also need to setup a [mock integration](https://zeit.co/dashboard/integrations/create) on ZEIT's website. For the UI Hook URL use `http://localhost:3000` and make the project private. Then, add the integration to your account and go to the integration page.

Once you have everything downloaded, you can download dependencies and start the project.

```sh
# install depdencies
cd ui
npm install

# start the development server
cd ..
now dev -p 3000
```

You can now refresh the integration page and you should see the local version of your UI Hook.

## Contibuting

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

I would love to see issues and pull requests to make this a better tool that works for people other than myself!

This project only works with rust's 2018 edition. Thus, you must have version 1.31 or later. Once you have rust installed, you can then run `cargo run` to see it in action. This will download and compile all the dependencies in development mode.

## [License](LICENSE.md)
