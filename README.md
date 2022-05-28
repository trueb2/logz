# LOGZ

This repo provides a crate with a Rust Logger implementation for use on an embedded device with Zephyr RTOS. `bindgen` and `cbindgen` are used to bind Rust calls to Zephyr RTOS's log2 implementation

## Build

Include this crate in another crate that needs a logger implementation and make the usual log macro calls like `log::trace!("2.0 * 2.0 = {}", 2. * 2.);`. With a high initialization priority, the global logger is initialized to call the matching LOG_DBG, LOG_INF, LOG_WRN, or LOG_ERR macro in the bridging C.

### Build Tools

In order to build there are several tools to install

* NRF Connect SDK
  * v1.9.1
* Rustup and Cargo
  * stable
  * v1.24.3 and v1.61.0
* Cross compiling targets
  * M4(F): `thumbv7em-none-eabihf`
  * M33(F): `thumbv8m.main-none-eabihf`
* Tools and Subcommands
  * `cargo install cargo-make,cbindgen`
  * `brew install llvm`
    * `llvm-config` needs to be in your `PATH`
    * `echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc`
    * `sudo xcode-select --install`
  
If you have all of that, then `cargo make test` should work.

## Example

In the example, we have a `blinky` application compiled for NRF52 or NRF53. The CMakeLists.txt of the sample project adds logz as a static library, exposes the headers to sample app, and runs the build with cargo whenever you build.