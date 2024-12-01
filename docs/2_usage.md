# Usage 



## Command
```text
Tracing / benchmarking long running applications (ie: streaming).

Usage: tracer [OPTIONS] [APPLICATION] [-- <ARGS>...]

Arguments:
  [APPLICATION]  Application to be run as child process (alternatively provide PID of running app)
  [ARGS]...      Optional program arguments (ignored with PID option)

Options:
  -p, --pid <PID>          PID of external process
  -n, --noui               Switch off UI - csv style output
  -a, --autoscale          Switch off auto-scale - this will use all available CPU/MEM in the graphs
  -r, --refresh <REFRESH>  Refresh rate in milliseconds [default: 1000]
  -o, --output <OUTPUT>    CSV output file
  -l, --log <LOG>          Custom log level: info, debug, trace [default: info]
  -h, --help               Print help
  -V, --version            Print version

```



## Examples


### Common - Simple

```shell
tracer application_name 
```
Note: application_name will be run as child process.
If going out of tracer app it will be killed.


### Runnig app

```shell
tracer -p 1234
```
This will connect to running application with PID 1234.
Going out of tracer app - will not kill the application.


### CSV output

```shell
tracer -o output.csv  application_name 
```

This will create output.csv file with time, cpu and memory readings.



### Additional parameters

```shell
tracer -o output.csv  application_name -- param1 param2
```

This will run application with additional parameters: `application_name param1 param2`



