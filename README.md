# Collatz sequence operations using WebAssembly

Built using the template at https://github.com/rustwasm/rust-webpack-template.

# Special instructions

I had to `apt-get install` a few packages:`libssl-dev` and `pkg-config`.

# Publishing

This is hosted via Firebase Hosting.

Run `npm run build` to:
  - compile the Rust crate and generate the Web Assembly module
  - web-pack the entire application into the `dist/` directory

Firebase Hosting is looking at the `public/` directory, which is just a symlink
to the `dist/` directory.
