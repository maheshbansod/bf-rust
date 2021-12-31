# bf-rust

A simple interpreter for the esoteric language [https://en.wikipedia.org/wiki/Brainfuck](Brainfuck).

It also has a basic debugger if you run the program with `-d` as a command line argument.

The interactive debugger supports the following commands:
- `data [i1 i2 ..]` : outputs the data at the given positions of the data array.
- `ip` : outputs the instruction pointer.
- `dp` : outputs the data pointer.
- `step [n]` : steps by one instruction or by `n` instructions if `n` specified.
- `run` : runs the program from the current `ip` till the end or till an error occurrs.
- `exit` : quit the debugger