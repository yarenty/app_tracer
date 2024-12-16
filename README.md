# TRACER

Monitor live application either as child process or separate PID, collecting /displaying stats ( cpu usage, memory usage).

_Note: For monitoring one-shot applications - see [https://github.com/yarenty/app_benchmark](https://github.com/yarenty/app_benchmark)._


## UI (TUI)

![uitracker screnshot](docs/img/uitracker.png)



## Build

```shell
cargo build -r
```


## Run 

Create example app:
```shell
cargo build --examples test_app
```

Run in txt mode and output persisted to out.csv file:
```shell
cargo run  -r -- -n -o out.csv /opt/workspace/app_tracer/target/debug/examples/test_app
```


## Usage

```shell
app-tracer 0.4.0
Tracing / benchmarking long running applications (ie: streaming).

USAGE:
    tracer [OPTIONS] [APPLICATION]

ARGS:
    <APPLICATION>    Application to be run as child process (alternatively provide PID of
                     running app)

OPTIONS:
    -h, --help                 Print help information
    -l, --log <LOG>            Set custom log level: info, debug, trace [default: info]
    -n, --noui                 No UI - only text output
    -o, --output <OUTPUT>      Name of output CSV file with all readings - for further investigations
    -p, --pid <PID>            PID of external process
    -r, --refresh <REFRESH>    Refresh rate in milliseconds [default: 1000]
    -V, --version              Print version information

```

## Example output

```log
cargo run -r -- -n -o out.csv /opt/workspace/app_tracer/target/debug/examples/test_app     
   Compiling app-tracer v0.4.0 (/opt/workspace/app_tracer)
    Finished release [optimized] target(s) in 2.98s
     Running `target/release/tracer -n -o out.csv /opt/workspace/app_tracer/target/debug/examples/test_app`
12:26:12.260 (t: main) INFO - tracer - Application to be monitored is: test_app, in dir /opt/workspace/app_tracer/target/debug/examples/test_app
12:26:12.261 (t: main) INFO - tracer - Refresh rate: 1000 ms.
12:26:12.261 (t: main) INFO - tracer - Output readings persisted into "out.csv".
12:26:12.261 (t: main) INFO - tracer - Starting with PID::15008
12:26:12.296 (t: main) INFO - tracer - Running in TXT mode.
12:26:13.298 (t: main) INFO - tracer - CPU: 0 [%],  memory: 2208 [kB]
12:26:14.303 (t: main) INFO - tracer - CPU: 0.0030129354 [%],  memory: 2208 [kB]
12:26:15.308 (t: main) INFO - tracer - CPU: 0.0054045436 [%],  memory: 2208 [kB]
12:26:16.309 (t: main) INFO - tracer - CPU: 0.0023218023 [%],  memory: 2208 [kB]
12:26:17.311 (t: main) INFO - tracer - CPU: 0.006252239 [%],  memory: 2208 [kB]
12:26:18.312 (t: main) INFO - tracer - CPU: 0.0036088445 [%],  memory: 2208 [kB]
12:26:19.317 (t: main) INFO - tracer - CPU: 0.0057060686 [%],  memory: 2208 [kB]
12:26:20.318 (t: main) INFO - tracer - CPU: 0.005099413 [%],  memory: 2208 [kB]
12:26:21.318 (t: main) INFO - tracer - CPU: 0.007175615 [%],  memory: 2208 [kB]
12:26:22.319 (t: main) INFO - tracer - CPU: 0.005251118 [%],  memory: 2208 [kB]
12:26:23.319 (t: main) INFO - tracer - CPU: 0.0021786916 [%],  memory: 2208 [kB]
12:26:24.321 (t: main) INFO - tracer - CPU: 0.006866733 [%],  memory: 2208 [kB]


```



## CSV persistence

Example output.csv file:

```csv
Time,Cpu,Mem
11:27:16.394591,0,2136
11:27:17.396917,0.004986567,2136
11:27:18.397440,0.006548807,2136
```


## [CHANGELOG](CHANGELOG.md)



## Activity


![Alt](https://repobeats.axiom.co/api/embed/c6bc985250b9c8e4f24600cf492f3806bec79346.svg "Repobeats analytics image")

