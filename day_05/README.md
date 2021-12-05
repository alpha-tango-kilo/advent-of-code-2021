# Day Five - Hydrothermal Venture

[**Link**](https://adventofcode.com/2021/day/5)

## Part one

I think you can tell by the `Display` and `Debug` impls that this challenge didn't go as well as expected.
Really liked the premise, really managed to continually mess up the co-ordinates.
Got there in the end, still pretty happy with my solution on the whole, though `Line::traversal_vec2` and `Line::steps_between` are pretty ugly I think

## Part two

Fortunately my solution already allowed for diagonals, literally all I had to do was remove the filter from line 8 of my part one solution
