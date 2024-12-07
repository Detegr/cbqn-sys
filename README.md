# cbqn-sys

A crate providing FFI bindings to [CBQN](https://github.com/dzaima/CBQN).

## Features

### shared-object

The default feature is `shared-object` which links to libcbqn.so. It can be built using `shared-o3` target. Use `RUSTFLAGS="-L /path/to/cbqn"` and `LD_LIBRARY_PATH=/path/to/cbqn/libcbqn.so` if the shared object is not in a system-wide path.

### No features

Building without features gives only the API to CBQN. Used with [cbqn-rs](https://github.com/Detegr/cbqn-rs) WASI backend for example.

## Pregenerated bindings

Pregenerated bindings can be found from `src/pregenerated.rs`. To recreate them, use bindgen-cli.

## License

Licensed either under GPLv3, LGPLv3 or MPL 2.0 following the licensing of [CBQN](https://github.com/dzaima/CBQN/).
