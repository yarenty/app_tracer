# Changelog

## 0.7.4
- fix output out/err files creation
Created two separate files for output:
{app}.out for stdout
{app}.err for stderr
Preserves the TUI functionality since we're not writing to stdout/stderr
Captures all output in separate files for later analysis
Uses the application name in the log files for easy identification
Maintains proper error handling with Rust's ? operator
The output files will be created in the current working directory and will contain all output from the monitored process. You can now view these files while the application is running or after it completes without interfering with the TUI.

## 0.7.3
- upgrade clap to 4.x 
- can run application with additional parameters: tracer application_name -- param1 param2
- redirected stdout to current stdout

## 0.7.2
- auto killing application created as subprocess
- adding CI/CD
- 
## 0.7.1
- removed unnecessary dependencies
- build size shrunk over 60%+

## 0.7.0
- simplified CPU history
- update TUI to latest version

## 0.6.0
- use summary readings from current PID and children PIDs

## 0.5.0
- autoscale by default - graphs looks much nicer ;-)

## 0.4.1
- support WSL
- fixed linux CPU id 

## 0.4.0
- support for apps that has they own parameters 
```./tracer -n -o aa.csv "/opt/workspace/app_tracer/target/debug/examples/test_app -a 10000 -b -c"```

## 0.3.0 
- no more 2 separate apps - simple flag -n (-noui) to use txt mode only
- added eyre for better error handling in runtime
- memory readings in KB
- optional output persistence to csv file
- 
## 0.2.0
- synchronized CPU output on all tabs

## 0.1.0
 - initial version
 - cli 
 - tui