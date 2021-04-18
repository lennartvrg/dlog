# dlog - Logging for microservice architectures

_________________

<div align="center">

[🌐 - Web app](https://app.dlog.cloud)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[🐱‍💻 - Repository](https://github.com/lennartvrg/dlog)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[📝 - Issues](https://github.com/lennartvrg/dlog/issues)
</div>


**dlog** is a logging platform designed for microservice architectures. It takes away the hassle of setting up your own
logging infrastructure by providing **drop-in** adapter for several programming languages. The **dlog** logging platform
can ingest thousands of logs every second and provides a fast and easy to use interface for developers to view their logs.

In the near future a **full text search** capable interface will be developed which can be used to analyse huge amounts
of log records. Additionally, it is planned to add **alerts** to the platform so that you can get informed when an error
occurred during operation.


## Features

_________________

This **dlog** adapter for python `logging` is...

- **Easy** to integrate for everyone who already uses the standard python `logging` module
- **Efficient** by batching many logs together and only ingesting after `1000` logs have accumulated or after `1` second
  has passed.
- **Non-blocking** by building upon a native library which offloads the ingestion to a background thread
- **Compatible** with serverless environments such as AWS Lambda which is often a problem for other logging frameworks


## Requirements
_________________

Since this module uses natively compiled code under the hood, ait only supports some operating systems and python versions.
If you desperatly need a version for another architecture / python version please open an issue on our
[📝 GitHub](https://github.com/lennartvrg/dlog/issues). Below you can find a matrix of our supported python versions and
architectures.

| Python version 	| Linux x64 	| MacOS x64 	| Windows x64 	|
|:----------------:	|:-----------:	|:-----------:	|:-------------:	|
| 3.8            	|     ✔️     	|     ✔️     	|      ✔️      	|
| 3.9            	|     ✔️     	|     ✔️     	|      ✔️      	|

## Getting Started

_________________

**1. Sign-in / Sign-up**

Head over to [app.dlog.cloud](https://app.dlog.cloud) and create an account or login into an existing account.
You can be part of multiple projects with a single account, so it not necessary to create a new account for every project.

**2. Create a project**

If you have a new account you should be greeted with an *Create Project* dialog. Pick a name for your new project and hit
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

<div align="center">

⚠️ **The API_KEY is only visible during the initial creation, so please take note and store it somewhere safe** ⚠️

</div>

With the API_KEY generated, you are ready to install the module and configure dlog.

**6. Install and configure dlog_python**

You can install the module using `pip install dlog_python` and configure it either for regular or serverless environments.
To see how this is done, please consult the [Basic Example](#basic) or [AWS Lambda Example](#serverless-functions).


## Examples
_________________


### Basic

For most use cases you can simply use dlog with the python `logging` module. For this, you will only need the `API_KEY`
to get started. The dlog logging handler is threadsafe and can be passed around inside your program as needed.

⚠️**NOTE:** This approach is not recommended for environments such as AWS Lambda. This is due to fact that Lambda
suspends the execution container once the request is finished. As this quest uses a background thread for ingestion logs,
that thread can no longer operate. To use dlog in AWS Lambda please look to the [Serverless functions](#serverless-functions)
section.

```python
import os
import logging
from dlog_python import DlogLogger

logger = logging.getLogger('main')
logger.addHandler(DlogLogger(os.environ['DLOG_API_KEY']))

logger.info("Hello World!")
```


### Serverless functions

To use this module in serverless environments such as AWS Lambda, you must use a special decorator method. The decorator
creates a standard python [Logger](https://docs.python.org/3/library/logging.html), injects it as the first parameter
and supports the standard logging method: `critical`, `error`, `warning`, `info`, `debug` and `log`. Below you can find
an example serverless handler.


```python
import os
import json
from dlog_python import with_dlog


@with_dlog(os.environ['DLOG_API_KEY'])
def lambda_handler(logger, event, context):
    logger.warning(f"Event body: { json.dumps(event) }")

    return {
        'statusCode': 200,
        'body': json.dumps('Hello from Lambda!')
    }
```

## API Reference

_________________


### Classes


| Class      | Parameter | Default    | Description                                                                                                                                                                                    |
|------------|-----------|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| DlogLogger | API_KEY   | *REQUIRED* | The `API_KEY` parameter must be a string containing the API_KEY for dlog. It is strongly recommended to not store the API_KEY in your source control, but rather in an environmental variable. |

### Methods

| Method     | Parameter | Default           | Description                                                                                                                                                                                    |
|------------|-----------|-------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| @with_dlog | API_KEY   | *REQUIRED*        | The `API_KEY` parameter must be a string containing the API_KEY for dlog. It is strongly recommended to not store the API_KEY in your source control, but rather in an environmental variable. |
|            | LEVEL     | `logging.WARNING` | The `LEVEL` parameter is the minimum level a log event must be so that it will be recorded by dlog. This parameter accepts as logging level as defined in the `logging` python module.         |


