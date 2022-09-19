# Changelog

# TODO
- update TUI to latest version ( use spans, etc..)

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