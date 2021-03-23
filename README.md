# 🦀Rustboard
## Imageboard built using [Rocket](https://rocket.rs/), [Tera](https://github.com/Keats/tera) and [Dgraph](https://dgraph.io/)

* All data is stored as highly relational nodes under dgraph
* Rocket handles everything else even down to SSR
* Rendering is done with rocket contrib's templating feature utilising Tera templates
* Goal is to have almost everything rendered on the server before sending anything to the client
* Tailwind used for styling


---
## Dependencies
* Rust
* Yarn (or npm)
* Dgraph
* Docker and docker-compose

---
## Building for development
If you don't have Rust and Cargo installed refer to [this site](https://rustup.rs/)

After cloning the repo make sure your rustc branch is set to nightly
`rustup show`

If not, run `rustup override set nightly` to set it locally in this repo's folder

After that run both `cargo build` and `yarn` (or `npm i`) to download and compile all dependencies

To start up your Dgraph containers run `docker-compose -f docker-compose.dev.yml up -d`

While your Dgraph containers are starting up go ahead and start the css processor in a separate terminal instance with `yarn css:watch` or `npm run css:watch`

Finally start the Rocket instance with `cargo run`

---
## Building for production

**WIP**

---
## License 

**MIT**