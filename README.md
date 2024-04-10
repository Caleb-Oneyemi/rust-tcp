# rust tcp

A sample implementation of a tcp server and client in rust. Client sends request to server using user supplied input via the command line and server responds with `Hello ${input}`

## Getting started

Here's how to run the project on your local machine for development and testing purposes. First Ensure you have rust tooling installed (https://rustup.rs/), then:

cd into project:

```shell
$ cd rust-tcp
```

run server:

```shell
$ cargo run -p server
```

run client:

```shell
$ cargo run -p client input
```
