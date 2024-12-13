# Advent of Code 2022

Solutions for the Advent of Code 2022.
I have not written a solution for all of them, the missing stars are:

- Day 16, part 2
- Day 17, part 2
- Day 22, part 2

I participated in the Advent of Code mainly as a learning experience. 
I did not manage to solve all the problems on my own. 
Three(?) days were too difficult for me.
When I wasn't able to solve it myself I looked for hints or even full solutions online.
After understanding the solution I would then code up my own solution.
Generally speaking, I found the optimization and path searching problems the most difficult.
It had been a while solving these kind of problems, but they do get easier the more you solve.

Day 19 has a Python and a Rust implementation. 
Day 19 I did not solve on my own, but took inspiration from a Rust solution posted to the subreddit that solved it in a few dozen milliseconds.
I cannot find that specific solution anymore, but there are similar solutions [here](https://old.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions).
I implemented the search in Python and it was super slow, like 30+ seconds slow.
This surprised me, because up to this point Python had been decently fast each day taking at most a few seconds.
Using Pypy did improve the running time, but it still took a long time.
So I also wrote a version in Rust, with very similar logic, to see if the slowdown really was introduced by using Python and not due to some
differences in my implementation and the implementation that I took inspiration from.  
And... it was just Python being slower.
The Rust version runs instantly for me, so I did not bother timing it precisely.
My curiosity here was satisfied :)
