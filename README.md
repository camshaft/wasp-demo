# wasp-demo

## Installation

* Install rust:
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  rustup install nightly
  rustup default nightly
  rustup target add wasm32-unknown-unknown
  ```

* Install [wasp](https://github.com/yaedo/wasp-cli)

* Install cargo-watch (optional):
  ```bash
  cargo install cargo-watch
  ```

## Usage

In one terminal we will start the build process. This will recompile the project any time a file changes. Run the following command:

```bash
make watch
```

In another terminal we will run the actual `wasp` server. This will hot-reload any changes made. Run the following command:

```bash
make run
```

Visit http://localhost:5000 and you should see `Hello, World!`. You can add a query parameter to change `World`: http://localhost:5000?Wasp.

## Making changes

Open `src/lib.rs` and make any changes you'd like. Refresh your browser tab and you will see the changes reflected in the browser.

The `HttpRequest` and `HttpResponse` types come from the [http crate](https://github.com/hyperium/http). Check out their docs on how to manipulate requests/responses.

## Deploying

Once you're happy with your changes deploy it for the world to see.

Let's start by creating the app throught the wasp api:

```bash
curl --data-binary '{"host": "my-wasp-app.wasp.website"}' https://api.wasp.ws/hosts
```

Next, upload your wasm binary:

```bash
curl --data-binary @./target/wasm32-unknown-unknown/release/app.wasm https://api.wasp.ws/compile
```

This should return a hash that points to your uploaded module. You can use this to configure your newly-created app.

```bash
curl --data-binary '{"module": "HASH", "function": "run"}' https://api.wasp.ws/hosts/my-wasp-app.wasp.website
```

Assuming everything worked you should be able to visit http://my-wasp-app.wasp.website and see your app running.

You can also create an app that uses your own domain name. Just replace `wasp.website` with your own `tld` and configure a CNAME entry to `route.wasp.ws`.