nRF52840 sample
===============


First of all you must install the `probe-run` utility.
```sh
cargo install probe-run
```


Add the cortex-m4 target.

```sh
rustup target add thumbv7em-none-eabihf
```

Create the configuration file ```.cargo/config.toml```.
The content of the file is the following:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip nRF52840_xxAA  --defmt"
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]

[build]
target = "thumbv7em-none-eabihf"
```
