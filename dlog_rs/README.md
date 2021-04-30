# dlog - Logging for microservice architectures

_________________

<div align="center">

[📦 - Crates.io](https://crates.io/crates/dlog_rs)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[🌐 - Web app](https://app.dlog.cloud)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[🐱‍💻 - Repository](https://github.com/lennartvrg/dlog/tree/master/dlog_rs)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[📝 - Issues](https://github.com/lennartvrg/dlog/issues)
</div>


**dlog** is a logging platform designed for microservice architectures. It takes away the hassle of setting up your own
logging infrastructure by providing **drop-in** adapter for several programming languages. It can ingest thousands of
logs every second and provides a fast and easy to use interface for developers to analyse their logs. Logs
can be observed in real-time or searched via a **full text search** capable query interface.


## Features

_________________

This **dlog** adapter for the rust [`log`](https://crates.io/crates/log) crate is...

- **Easy** to integrate for everyone who already uses the standard Rust [`log`](https://crates.io/crates/log) crate
- **Efficient** by batching many logs together and only ingesting after `1000` logs have accumulated or after `1` second
  has passed.
- **Non-blocking** by building upon a native library which offloads the ingestion to a background thread


## Requirements
_________________

This crate depends on the [`log`](https://crates.io/crates/log) crate as it is an adapter for said crate. It should work
with every recent version. During development Rust 1.51.0 and the stable toolchain were used.

## Getting Started

_________________

**1. Sign-in / Sign-up**

Head over to [app.dlog.cloud](https://app.dlog.cloud) and create an account or login into an existing account.
You can be part of multiple projects with a single account, so it not necessary to create a new account for every project.

**2. Create a project**

If you already have a new account, you should be greeted with an *Create Project* dialog. Pick a name for your new project and hit
*Create*. If you already have a project you want to use, click it in the sidebar or in the general project list. To go
to the project list, please click the *dlog.cloud* logo in the upper left corner. Here you can also create a new project.

**3. Create a new application**

Once you select a project, you should be forwarded to the *Dashboard* section. Here you can see of a list of all the apps
inside the current project. Now you can either select an application from the list or create a new one by clicking the
plus icon on the right side. Pick a name for your new app and hit *Create*.

**4. Create a new service**

Once you click on an app, you will see the per-app settings and statistics. You can either select an existing service
from the list or hit the plus icon on the right side. Pick a name for your new service and hit *Create*.

**5. Create a new API_KEY**

Once you click on a service, you will see a list of API_KEYs. It is possible for an service to have more than one API_KEY
in case you want to differentiate between different deployment environments such as `dev` or `prod`. To create a new
API_KEY simply click to plus icon on the right side, pick a name and hit *Create*.


With the `API_KEY` generated, you are ready to install the module and configure dlog.

**6. Install and configure dlog_rs**

Ensure that the `Cargo.toml` contains something like this

```toml
[dependencies]
log = "0.4.14"
dlog_rs = "1.0.0"
```

To see how to configure `dlog_rs`, please consult the [Basic Example](#basic).


## Examples
_________________


### Basic

For most use cases you can simply use dlog with the `log` crate. For this, you will only need the `API_KEY`
to get started.

```rust
#[macro_use]
extern crate log;

use std::env;

fn main() {
    dlog_rs::configure("<API_KEY>");

    info!("Hello, world!");

    // This is not required but it makes sure that all
    // logs are flushed when the application exits.
    log::logger().flush();
}
```

### `API_KEY` from environmental variable or with custom log level

You can also load the `API_KEY` from an environmental variable using the builder pattern. Here you can also set the
minimum log level which should be logged to dlog.

```rust
#[macro_use]
extern crate log;

use std::env;

fn main() {
     dlog_rs::Builder::new()
         .with_env_api_key("DLOG_API_KEY")
         .with_level(log::Level::Info)
         .build();

    info!("Hello, world!");

    // This is not required but it makes sure that all
    // logs are flushed when the application exits.
    log::logger().flush();
}
```

## API Reference

_________________


### Methods

| Method    | Parameter | Default    | Description                                                                                                                                                                                    |
|-----------|-----------|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| configure | API_KEY   | *REQUIRED* | The `API_KEY` parameter must be a string containing the API_KEY for dlog. It is strongly recommended to not store the API_KEY in your source control, but rather in an environmental variable. |
