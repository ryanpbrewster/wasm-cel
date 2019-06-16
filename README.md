# CEL parser + evaluator

* Q: What is CEL?
* A: https://github.com/google/cel-spec

* Q: What is this?
* A: A parser and evaluator for CEL, written in Rust, compiled to Web Assembly, and deployed into a browser environment.

* Q: How do I use it?
* A: `npm run start` and visit `localhost:8080` in your browser.

## Web Assembly

Built using the template at https://github.com/rustwasm/rust-webpack-template.

### Special instructions

I had to `apt-get install` a few packages:`libssl-dev` and `pkg-config`.

## Publishing

This is hosted via Firebase Hosting.

Run `npm run build` to:
  - compile the Rust crate and generate the Web Assembly module
  - web-pack the entire application into the `dist/` directory

Firebase Hosting is looking at the `public/` directory, which is just a symlink
to the `dist/` directory.
