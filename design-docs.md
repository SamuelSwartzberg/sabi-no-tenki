# Sabi no Tenki

A terminal command line weather client with fine-grained options and a pretentious and possibly incorrect japanese name.

## Synopsis

tenki \[options] \[time]

## Time

```
time = [start\[:end]\[:step]]] | shorthand-values
start = time-value
end = time-value
step = time-value
time-value = time,unit
time = 0...23
unit = 'd'|'h'
shorthand-values = 'today'|'week'|'weekend'|'next-week'

  no value - now
  1h - in one hour
  5d - in 5 days
  2w - in two weeks
  1h:10h - in the 9 hours starting one hour from now
  1h:10h:3h - in the in the 9 hours starting one hour from now every three hours
  :6d:2d - in the next six days, every second day


Leaving out step is usually fine - if you specify a h value for any of the values, it will presume hour-based stepping, otherwise it will presume day-based stepping.

```

## Options

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
  And so on for kind of weather, wind, wetterwarnungen etc. etc.
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
  --significant-figures FIGURES
    give this amount of significant figures (e.g. --significant-figures 1 -> 25.1 C)
  --emoji / --text
    Show things such as current weather as emoji, text, or both (e.g. 'Rainy', '🌧', or '🌧 Rainy')
```

## Output

outputs in CSV-like syntax, unless `-h` is specified. (complies with UNIX philosophy of using text as the main tool of data transmition and being combinable with other programs) 

## Config

reads from a `$XDG_CONFIG_HOME/sabi-no-tenki` file. Formatted in TOML. Set options here as `option=value`.

## Caching

If caching is enabled, will cache responses and reuse them if still considered valid. On partial cache hit, will revalidate, as the cost of revalidating some vs all the cache are nearly the same.

## Program flow
  ```
  Parse input  
  -> if invalid or --help -> return the relevant thing
  -> set the output parameters/flags
  -> set the requestBuilder parameters/flags
  Parse options file
  -> set the output parameters/flags
  -> set the requestBuilder parameters/flags
  build a request (API-specific)
  -> check cache
  recieve response
  -> save response in cache
  parse response (API-specific) into sequence of Weather structs \\NOTE: if we're combining APIs, we might need to do this multiple times
  reade output flags, build output from Weather structs
  ```
## Weather structs
  Main logic struct: Weather for a specific time unit and relevant data  
   parse information like types of weather ("cloudy", "sunny", etc.) into enums, to allow for easy localization & different display when need arises
  

                                        
