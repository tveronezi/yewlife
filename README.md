# yewlife 

Conway's game of life with https://yew.rs

Demo here https://yewlife.tveronezi.workers.dev/

## Dependencies

Runtime:
```shell
user:~$ cargo version
cargo 1.60.0 (d1fd9fe 2022-03-01)

user:~$ docker --version
Docker version 20.10.14, build a224086

user:~$ npm --version
8.3.1
```

One-time commands:
```shell
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
npm install -D tailwindcss
```

## How to run it in development mode?

Terminal one:

```
npx tailwindcss -i ./tailwind-input.css -o ./tailwind.css --watch
```

Terminal two:

```shell script
trunk serve
```

Now open http://localhost:8080/
