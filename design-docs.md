A terminal command line weather client with fine-grained options.

# Features
Different flags (options)
Abstraction/Hooks for different APIs

# Ideas
syntax for date selection:  
  start\[:end\[:step]], e.g. 1h, 5d, 2w, 1h:10h, :12h:3h 3d:6d, with shorthand for keyword values like today, this week, this weekend  
default output in CSV-like syntax, unless specified (complies with UNIX philosophy of using text as the main tool of data transmition and being combinable with other programs)  
flags for program:  
```
  --api API 
    Use specified api
  -h, --human-readable, 
    Use human-readable text instead of CSV-like syntax
  --all, --no-all
    Include or exclude all possible metrics
  --temperature, --no-temperature
    Include or exclude temperature
  --precipitation, --no-precipitation
    Include or exclude temperature
  And so on for wind, wetterwarnungen etc. etc.
  -a, --ascii
    Include an ascii graphic for every requested unit
  --graph METRICS
    Show an ascii graph for the requested metrics
    e.g. for preciptation
    *
    **
    **  *        *
    ******    *  *
    <inserrt times here>
  --cache-duration DURATION
    Specify the duration to cache previous results
  --cache-revalidate-partial, --no-cache-revalidate-partial
    If only partial result in cache, refetch all data
```
config file: In TOML  
  options: cache duration  
Cache:  
  Cache should work partially, that is we store any result in cache in such a way that we can access it easily and combine it with data from a new request  
Program flow:  
  Main logic struct: Weather for a specific time unit and relevant data  
  Flow: Parse request + options in config, set output paramenters, figure out what we have to fetch => check cache, fetch from API => assemble into sequence of weather structs => Read output parameters, transform Weather structs sequence into output => print output  

                                        
