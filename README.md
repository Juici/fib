# fib

A basic fibonacci number calculator (and Command Line Interface!)

## Install Binary

```
cargo install fib
```

This will install the source code and compile it into a binary `fib` in your `.cargo/bin` directory. 

## Usage

```
$ fib 20
6765
$ fib {0..10}
0
1
1
2
3
5
8
13
21
34
55
```


## Time Complexity

This implementation runs in O(n) time. The fastest algorithms are O(log n). On my laptop, calculating the 100,000'th fibonacci term is near instaneous, but you may want to use an O(log n) implemetations for terms greater than one million.

Here are the time results from running the algorithm on my laptop (ARM architecutre, M1 macbook air).

| Fibonacci term | Time to run  |
| -------------- | ------------ |
| 100,000        | 0.15 seconds |
| 200,000        | 0.49 seconds |
| 300,000        | 1.05 seconds |
| 400,000        | 1.84 seconds |
| 500,000        | 2.84 seconds |
| 700,000        | 5.53 seconds |
| 1,000,000      | 11.3 seconds |

The exact commands I executed was `time fib 300000`. I think the reason the time complexity seems to grow faster than O(n) is because it's printing to screen. Or perhaps it's because the memory is also growing and referencing large chunks of memory slows everything down.


## Appendix - debugging

If you're having trouble running `fib` from terminal, make sure that `.cargo/bin` is in your PATH. 



