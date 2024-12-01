# Motivation

I work on both "one-shot" batch applicaitons and streaming style on-going applications. 
I don't really want to monitor them - for that I could use grafana with prometheus or any other collectors.

My issue is: I'm lazy and have no time to do this every time I need to deliver some new functionality and need to provide benchmarking results.
As my work is usually full end-to-end solution then simply monitoring it would be really useful, and again writing criterion like tests - not so useful.

That's why I created this tool and another form benchmarking batch applications (app_benchmarker).

It could be used with any application (as long as it doesn't need any special interactions!) and it would be really useful for me.

3 easy steps:
1. Install it with `cargo install tracer`. 
2. Run `tracer -o output.csv  application_name -- param1 param2`
3. Show the results in UI - for demo ! or save output to csv file - if you need results for analysis.



