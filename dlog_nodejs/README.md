# dlog - Logging for microservices &nbsp; ![npm](https://img.shields.io/npm/v/dlog-nodejs?color=blue)

<div align="center">

[📦 - npm](https://www.npmjs.com/package/dlog-node)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[🌐 - Web app](https://app.dlog.cloud)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[🐱‍💻 - Repository](https://github.com/lennartvrg/dlog/tree/master/dlog_nodejs)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[📝 - Issues](https://github.com/lennartvrg/dlog/issues)
</div>

**dlog** is a logging platform designed for microservice architectures. It takes away the hassle of setting up your own
logging infrastructure by providing **drop-in** adapter for several programming languages. It can ingest thousands of
logs every second and provides a fast and easy to use interface for developers to analyse their logs. Logs
can be observed in real-time or searched via a **full text search** capable query interface.

## Features

This **dlog** adapter for nodejs is...

- **Easy** to integrate for everyone who already uses logging via `console.*`
- **Efficient** by batching many logs together and only ingesting after `1000` logs have accumulated or after `1` second
  has passed.
- **Non-blocking** by building upon a native library which offloads the ingestion to a background thread
- **Compatible** with serverless environments such as AWS Lambda which is often a problem for other logging frameworks


## Requirements

Since this module uses natively compiled code under the hood, it only supports some operating systems and nodejs versions.
If you desperately need a version for another architecture / nodejs version please open an issue on our
[📝 GitHub](https://github.com/lennartvrg/dlog/issues). Below you can find a matrix of our supported nodejs versions and
architectures.


| NodeJS version | MacOS x64 | Windows x64 | Linux x64 | Linux arm64 (aarch64) | Linux arm (armv7) |
|:--------------:|:---------:|:-----------:|:---------:|:---------------------:|:-----------------:|
|    >=10.0.0    |     ✔️     |      ✔️      |     ✔️     |           ✔️           |         ✔️         |


## Getting Started

**1. Sign-in / Sign-up**

Head over to [app.dlog.cloud](https://app.dlog.cloud) and create an account or login into an existing account.
You can be part of multiple projects with a single account, so it is not necessary to create a new account for every project.

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

**6. Install and configure dlog-nodejs**

You can install the package using `npm install dlog-nodejs` or `yarn add dlog-nodejs` and configure it either for regular or serverless environments.
To see how this is done, please consult the [Basic Example](#basic) or [AWS Lambda Example](#serverless-functions).


## Examples


### Basic

For most use cases you can simply use dlog with the nodejs `console` logging methods. For this, you will only need the `API_KEY`
to get started. The `configure` method may only be called once and further invocation will result in a non-fatal error message.

⚠️**NOTE:** This approach is not recommended for environments such as AWS Lambda. This is due to fact that Lambda
suspends the execution container once the request is finished. As this module uses a background thread for ingestion logs,
that thread can no longer operate. To use dlog in AWS Lambda please look to the [Serverless functions](#serverless-functions)
section and use the `with_dlog()` higher order function.

#### JavaScript

```javascript
const dlog = require('dlog-nodejs')

dlog.configure(process.env.DLOG_API_KEY)

console.warn("Hello World!")
```


### Serverless functions

To use this package in serverless environments such as AWS Lambda, you must use a special wrapper method. The wrapper
ensures that the logs are being flushed to dlog at the end of the serverless function.


```javascript
const dlog = require('dlog-nodejs')

const handler = function (event) {
  console.log("AWS Lambda payload: " + JSON.stringify(event, null, 2))

  return {
      statusCode: 200,
      body: "Hello World"
  }
}

exports.handler =  dlog.with_dlog(process.env.DLOG_API_KEY, handler)
```

## API Reference


### Methods

| Method    | Parameter | Default    | Description                                                                                                                                                                                    |
|-----------|-----------|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| configure | API_KEY   | *REQUIRED* | The `API_KEY` parameter must be a string containing the API_KEY for dlog. It is strongly recommended to not store the API_KEY in your source control, but rather in an environmental variable. |
| with_dlog | API_KEY   | *REQUIRED* | The `API_KEY` parameter must be a string containing the API_KEY for dlog. It is strongly recommended to not store the API_KEY in your source control, but rather in an environmental variable. |

### TypeScript

TypeScript is supported for this package. The type definitions are part of the NPM package and don't require any
additional download.
