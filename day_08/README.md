# Day Eight - Seven Segment Search

[**Link**](https://adventofcode.com/2021/day/8)

## Part one

Oh no, oh god no.
They made part one doable so long as you can vaguely read the prompt and half-understand it.
I am not looking forward to what part two inevitably is going to be

## Part two

I did it! It took several hours, but I did it.
Trying to work out more and more deductions I could make about the segment configurations of digits to narrow down possibilities was hard work.
Too long was spent staring at segmented numbers.
I'm not sure if my solution has enough logic rules to work on any input, but I had enough to complete the challenge for my input.
I'm quite sure my solution is not the most allocation-efficient out there, but what matters to me at this point is that it works, despite me reading today's prompt and being like "I'm not going to be able to do that"

I used three rules to reduce the number of possible segments each character could be:
1. The possible segments for each unique segment-count digit are known
2. 5 letter/segment digits must all share 3 segments, being the top, middle, and bottom
3. 6 letter/segment digits must all share 4 segments, being the top, top left, bottom, bottom right

Based on that my program then makes any deductions it can - i.e. if there's one letter that can only be one possible segment, then no other letters can be that segment.
Repeat until stable.

Those 4 steps were enough to solve for my input but I don't know if it's general enough to solve any input
