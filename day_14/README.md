# Day Fourteen - Extended Polymerisation

[**Link**](https://adventofcode.com/2021/day/14)

## Part one

Still having not completed the utter shitshow I made of yesterday's challenge, I decided I preferred the look of this one.
Little did I know this was going to be a 'find the efficient solution' problem and so my implementation is lovely and naive, but relatively easy to write

## Part two

Well I did end up writing a constant-memory recursion based approach to the problem, but that wasn't going to work either.
Then I finally realised the map/count based approach and hastily wrote that.
I will admit to panicking when I thought the order of the polymer might matter in order to correctly count the occurences of each character, and will give full credit another Rust solution (that was already a similar 'shape' to mine) that showed me how trivial of an issue this was to actually fix.
It was also useful in debugging where my code went wrong - I was keeping the count from the previous iteration and adding it to the new count, which was generally speaking an awful idea

[Credit](https://github.com/AxlLind/AdventOfCode2021/blob/7ed94ac780b564eb20539ab0d78c8cdeed35cf4e/src/bin/14.rs)
