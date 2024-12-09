# Advent of Code 2024

My attempt at learning Rust by doing the advent of code 2024.

# Notes

## Day 6

In day 6, the initial full solution had a runtime of 18.73s on my machine.
After only checking for adding obstructions in places where the guard travelled in the first run, runtime was reduced to
~4 seconds.

## Day 7

Part 1 could be done using a single bit to define the operator.
Part 2 requires 2 bits, since there's 3 operator.
The operands would look something like:
00 -> add
01 -> concat
10 -> multiply

However, we need to make the parameter number not contain 11, since there's no 4th operator.

Got it working this way, only the runtime is very long (>10 min on my laptop).

### Performance

After looking into day 7, people suggested a recursive function should be significantly faster.

With the initial implementation, it runs in about 5 seconds on a debug build, and closer to 2 second for a production 
build.

With culling the results that have already exceeded the wanted result early (after fixing a silly bug with naming 2 
variables the same), runtimes are approx. 2.8 seconds for debug and 850ms for release. 