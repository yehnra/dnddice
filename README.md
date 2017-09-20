# dnddice
### A simple dice-rolling command line program.
dnddice is still in development (see TODO's in src/main.rs).

```
[dnddice]
Usage: dnddice [NUMBER_OF_DICE] [NUMBER_OF_DICE_SIDES]
Example: dnddice 2 20         # rolls 2d20's
Roll some dice.
  --csv                 output in .csv format, use `>' to pipe output to a file
  --help                display this help and exit
  --version             output version information and exit
```

## Examples:

```
# cargo
$ cargo run -- 3 6          # roll 3x d6
$ cargo run -- --csv 10 12  # roll 10x d12, output in .csv format

# binary
$ dnddice 5 10              # roll 5x d10
$ dnddice --csv 6 20        # roll 6x d20, output in .csv format
```

## Version and Licensing Info:

```
[dnddice 0.1.0]
Copyright (C) 2017 Baerlabs
License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>
dnddice is free to use, study, change, and/or redistribute.
There is no warranty, to the extent permitted by law.

Written by yehnra
```
