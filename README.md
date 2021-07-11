```
Sabi no Tenki 0.0
Sam S. <me@samswartzberg.com>
A terminal command line weather client with fine-grained options and a pretentious and possibly
incorrect japanese name. Can also be configured via $XDG_CONFIG_HOME/tenki_config.yaml
(.config/tenki_config.yaml). Keys are the same names as options, but using underscores instead.
Boolean flags cannot be set via config, as it would not be able to override them from the command
line.

USAGE:
    tenki [FLAGS] [OPTIONS] [time_list]

ARGS:
    <time_list>    The time span to fetch weather for:
                   time = [start[:end][:step]]] | shorthand-values
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
                   1h:10h:3h - in the in the 9 hours starting one hour from now every three
                   hours
                   :6d:2d - in the next six days, every second day
                   
                   
                   Leaving out step is usually fine - if you specify a h value for any of the
                   values, 
                   it will presume hour-based stepping, otherwise it will presume day-based
                   stepping.

FLAGS:
    -a, --ascii-image        Include an ascii image for every requested unit
        --emoji              Show things such as current weather as emoji (e.g. '🌧️'). Can be
                             combined with --text if both are desired (e.g. '🌧️ Rainy').
        --help               Prints help information
    -h, --human-readable     Use human-readable text instead of CSV-like syntax
        --labeled-columns    Show the names of the metrics at the beginning of the first column.
        --text               Show things such as current weather as text (e.g. 'Rainy'). Can be
                             combined with --emoji if both are desired (e.g. '🌧️ Rainy').
    -V, --version            Prints version information
        --week-sat           Week starts Saturday
        --week-sun           Week starts Sunday

OPTIONS:
        --api <API>
            Use specified api. Not all requested metrics are available for all apis. Currently
            unimplemented.

        --cache-duration <DURATION>        Specify the age of cached results you accept.
    -g, --graph <METRICS>
            Show an ascii graph for the requested metrics. Currently unimplemented.

    -l, --location <LOCATION_LIST>
            Use the specified location. Best specified in options file for default location. List is
            separated by colons (:)

        --metrics <METRIC-LIST>
            Specify a list of metrics you would like to recieve, as a non-spaced comma-separated
            list. Not all requested metrics are available for all apis.

        --significant-figures <FIGURES>
            give this amount of significant figures (e.g. --significant-figures 1 -> 25.1 C)
```