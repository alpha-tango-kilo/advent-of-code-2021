# Day Thirteen - Transparent Origami

[**Link**](https://adventofcode.com/2021/day/13)

## Part one

This, for whatever reason, has been my single most hated and challenging task.
My original implementation used a grid (`Vec<bool>` that behaved like it was 2D) and iterators over co-ordinates to function, and it was just so damn error prone I really struggled to make it work, getting confused over the highest x co-ordinate, rows, columns, whether a fold along x was vertical or horizontal, etc. etc.
It basically just compounded into a huge mess

I ended up re-writing the solution basically from scratch (keeping only `Fold` and `Axis`) just using a set of points and it's resulted in a really elegant looking solution that actually f*cking works.
Nearly two days after the task was released 🥲

## Part two
