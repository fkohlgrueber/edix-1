# edix-1: Mixing Fonts

[![LOC](https://tokei.rs/b1/github/fkohlgrueber/edix-1)](https://github.com/fkohlgrueber/edix-1)

Editor experiment 1: 

TODO: Description...

## Next steps
- Alignment (ref: Elastic tabstops)
- Line-wrapping for comments
- Block-layout
- ...

### Build

Make sure to have `Rust` and `wasm-pack` installed. Then run the following commands from the project folder:

```
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
```

You can locally host the experiment, e.g. by using Python's simple http server:

```
python3 -m http.server
```
