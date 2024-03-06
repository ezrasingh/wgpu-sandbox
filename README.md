# Wgpu Sandbox

A place to experiment with Wgpu and Shader programming.

## Getting Started

Make sure you have [Trunk](https://trunkrs.dev/) installed in order to build the WASM client and run the dev server.

```shell
cargo install trunk
```

Install the nightly toolchain to enable function-call syntax for signals in Leptos

```shell
rustup toolchain install nightly

# ? enable for this project only
rustup override set nightly
```

Add the wasm32 target

```shell
rustup target add wasm32-unknown-unknown
```

Run the application using

```
trunk serve
```
