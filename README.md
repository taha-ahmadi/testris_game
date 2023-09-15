I've just seen tetris movie and decide to write one for my self.
It was a good movie, I saw some tutorials to understand how does it work make it using Rust and WASM.

## Building

Make sure you have [Rust](https://www.rust-lang.org) installed and
[wasm-pack](https://rustwasm.github.io/wasm-pack/). To build this project, run:

```
$ wasm-pack build --target web
```

To run this project, you need a static file server. You can install `sfz` with
cargo:

```
$ cargo install sfz
```

Now, start your static file server and open `index.html`:

```
$ sfz -r
```
