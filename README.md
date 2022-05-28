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

In the example, we have a `blinky` application compiled for NRF52 or NRF53. The CMakeLists.txt of the sample project adds library that needs a logger, but is built as a static library for the main.c of Zephyr RTOS. The example project includes logz as a crate dependency and adds a lib/logz submodule. The submodule holds the C headers and src file to include in the example CMakeLists.txt.

The full example is here: https://github.com/trueb2/logz-example-rs


In the example CMakeLists.txt, we build the project with a static library and the bridge.c of logz
```cmake
cmake_minimum_required(VERSION 3.20.0)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(blinky)

### EXAMPLE STATIC LIBRARY :: BEGIN ###

# Allow app to include headers from the library including normal or bindgen headers
set(LOGZ_EXAMPLE_LIB_SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib/logz-example-rs)
set(LOGZ_LIB_SRC_DIR ${LOGZ_EXAMPLE_LIB_SRC_DIR}/lib/logz)
# Cortex-M4F (NRF52)
# set(LOGZ_EXAMPLE_LIB_DIR ${LOGZ_EXAMPLE_LIB_SRC_DIR}/target/thumbv7em-none-eabihf/release)
# Cortex-M33F (NRF53)
set(LOGZ_EXAMPLE_LIB_DIR ${LOGZ_EXAMPLE_LIB_SRC_DIR}/target/thumbv8m.main-none-eabihf/release)
include_directories(AFTER ${LOGZ_EXAMPLE_LIB_SRC_DIR}/include ${LOGZ_EXAMPLE_LIB_SRC_DIR}/include/generated ${LOGZ_LIB_SRC_DIR}/include/generated)
add_library(liblogz_example_rs STATIC IMPORTED GLOBAL)
set_target_properties(liblogz_example_rs PROPERTIES IMPORTED_LOCATION ${LOGZ_EXAMPLE_LIB_DIR}/liblogz_example_rs.a)
# Always let cargo decide if the library should be rebuilt
add_custom_target(
  liblogz_example_rs_target
  ALL
  BYPRODUCTS ${LOGZ_EXAMPLE_LIB_DIR}/liblogz_example_rs.a
  WORKING_DIRECTORY ${LOGZ_EXAMPLE_LIB_SRC_DIR}
  COMMAND cargo make build
  )
add_dependencies(app liblogz_example_rs_target)
# A couple Cortex M math functions get multiple definitions
target_link_libraries(app PRIVATE ${LOGZ_EXAMPLE_LIB_DIR}/liblogz_example_rs.a -Wl,--allow-multiple-definition)

### EXAMPLE STATIC LIBRARY :: END ###


### The logger bindings still need to be compiled for zephyr, which is why we still have to build the static lib and bridge
target_sources(app PRIVATE src/main.c ${LOGZ_LIB_SRC_DIR}/src/bridge.c)
```

This makes sure that the log macro calls will be piped to an initialized Logger instance that writes its logs to the Zephyr LOG2 implementation as a string formatted in Rust

In Zephyr, we call a Rust library function, `example_foo()`
```C
void flip_timer(struct k_timer* timer) {
	LOG_INF("LED: %d", (int) on);
	gpio_pin_set(led_dev, PIN, (int)on);
	on = !on;
	example_foo();
}
```

In Rust, we format the logs and invoke the LOG2 macros
```rust
#[no_mangle]
pub extern "C" fn example_foo() {
    log::trace!("Foo");
    log::debug!("Bar");
    log::info!("Fizz");
    log::warn!("Buzz");
    log::error!("Fizzle");
}
```
```C
void log_inf(const char *restrict msg)
{
    LOG_INF("%s", msg);
}
```

In the UART, RTT, or whatever you have configured in your Kconfig, we get normal logs.
```
[00:04:32.166,320] <inf> rs: logz_example_rs: Fizz
[00:04:32.166,351] <wrn> rs: logz_example_rs: Buzz
[00:04:32.166,381] <err> rs: logz_example_rs: Fizzle
[00:04:32.416,198] <inf> blinky: LED: 0
[00:04:32.416,259] <inf> rs: logz_example_rs: Foo
[00:04:32.416,290] <inf> rs: logz_example_rs: Bar
[00:04:32.416,320] <inf> rs: logz_example_rs: Fizz
[00:04:32.416,351] <wrn> rs: logz_example_rs: Buzz
[00:04:32.416,412] <err> rs: logz_example_rs: Fizzle
[00:04:32.666,198] <inf> blinky: LED: 1
[00:04:32.666,259] <inf> rs: logz_example_rs: Foo
[00:04:32.666,290] <inf> rs: logz_example_rs: Bar
[00:04:32.666,351] <inf> rs: logz_example_rs: Fizz
[00:04:32.666,381] <wrn> rs: logz_example_rs: Buzz
[00:04:32.666,412] <err> rs: logz_example_rs: Fizzle
[00:04:32.916,229] <inf> blinky: LED: 0
[00:04:32.916,290] <inf> rs: logz_example_rs: Foo
[00:04:32.916,320] <inf> rs: logz_example_rs: Bar
[00:04:32.916,351] <inf> rs: logz_example_rs: Fizz
[00:04:32.916,381] <wrn> rs: logz_example_rs: Buzz
[00:04:32.916,442] <err> rs: logz_example_rs: Fizzle
```