# Day Six - Lanternfish

[**Link**](https://adventofcode.com/2021/day/6)

I could tell from reading the prompt that this was a 'you have to find the smart solution or your computer will melt' problems, like the adapters from last year.
I was defeated by the challenge last year but was determined I'd do it this year.
I realised that you could store the number of fish at each stage in their lifecycle rather than simulating each fish individually.
Hey presto, a fast solution!

I did try and make my solution *slightly* general in case part 2 varies the time between babies or the number of babies, something like that.
It wasn't actually general over the timeframes but I could probably have made it if I needed to.
Fortunately, it was just running a longer simulation so I didn't have to do any further work! 😄

I'm also glad I actually looked in the standard library to find [`rotate_left`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left) existed, instead of doing a repeat of day 1 and not realising [`windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) existed, as that greatly simplified things
