# Changelog

## 0.8.0 (The Great Ratening)

*   **Major "Improvement":** We've swapped out `tui` for `ratatui`. Yes, you heard that right. We've traded one library for another, because why not? Who needs stability when you can have... *change*?  It's like switching from a slightly leaky bucket to a bucket made of *slightly* different materials. This is progress, people! Pure, unadulterated progress! The world will never be the same. (Probably.)
*   **UI Overhaul (of Sorts):** The UI, in its infinite wisdom, has been "modernized". What does this mean? Nobody knows. Prepare for the layout to be roughly the same, but slightly different, as if it had been subtly rearranged while you weren't looking. Embrace the chaos, or just pretend nothing has changed. Your call.
*   **Performance Boost (Debatable):** We've heard rumors that `ratatui` might be faster than `tui`. Or maybe it's slower.  Frankly, we're not sure, and neither should you be. Benchmarks? Who needs benchmarks when you have *feelings*? We're running on vibes here. If it feels faster to you, then it's faster.
*   **Crossterm Embrace:** Now, we have full `crossterm` support. So we got a new dependency. It will be better. We hope so. If not, you should be aware of it.
* **Termion RIP**: Good bye termion, we will not miss you.
*   **Bug Fixes (Possibly):**  Since we changed a fundamental UI library, we've undoubtedly fixed some bugs. We've also probably introduced some new, exciting ones.  It's a bug lottery! Will your favorite feature work? Or will it now cause the terminal to spontaneously combust?  Only one way to find out!  Please test generously and let us know. Or don't. We're not your boss.
* **Log Enhancements** Now we are logging in way that it is visible in TUI mode.
* **Panic Handling** App will not exit from panic, it will log it in log view.
* **General Refactoring** Code was cleaned up after replacing tui with ratatui.
* **No More Black Boxes:** The old `tui` was too mystical. Too magical. With `ratatui`, we've removed some of that unexplainable stuff. Now, your terminal app is powered by slightly-less-unexplainable things.
* **New dependecy**: one of old dependecy was replaced with new one.



## 0.7.7
- upgrade termion to 4.0.3
- enhance terminal handling with latest termion features
- fix terminal setup to use proper alternate screen creation
- improve TUI responsiveness and color support
- maintain compatibility with existing terminal features

## 0.7.6
- upgrade thiserror to 2.0.11
- improve error handling with latest thiserror features
- maintain backward compatibility with existing error types

## 0.7.5
- upgrade env_logger to 0.11
- update logger configuration to use new env_logger API
- switch to termion for colored logging output:
  - bright cyan timestamps
  - bright magenta thread names
  - yellow module paths in brackets
  - improved level colors (bright red errors, yellow warnings, bright green info)
- improve error handling in logger setup

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