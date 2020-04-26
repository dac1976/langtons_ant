# Langton's Ant Simulator #
## Overview ##
For some time I have wanted to learn the Rust programming language. I've toyed with the idea on and off for the last 2 years. At long last I made the effort to write a non-trivial, but still simple project, to get to grips with the fundamentals.

Hence, I decided to do my own implementation of [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant). My version includes the extension to multiple colours, one randomly chosen colour per direction in the defined movment rule.

The project creates a simple console application that prompts the user for input of control parameters.

The code makes use of core standard Rust library code as well as the Piston crate to provide access to a 2D graphical rendering window.

I have included the full source code, cargo TOML file and also the VSCode workspace and support files.

## Examples ##
Good path rules to try...

RL 
This is the original classic rule.

RLR
Grows chaotically. It is not known whether this ant ever produces a highway. 

LLRR
Grows symmetrically.

LRRRRRLLR
Fills space in a square around itself. 

LLRRRLRLRLLR
Creates a convoluted highway. 

RRLLLRLLLRRR
Creates a filled triangle shape that grows and moves. 

RLLR
Symmetrical expansion with a mix of space invader and skull patterns.

RLRRRL
Spreads out then makes a square path around the perimeter, which occasioanlly grows then gets locked back into a square path around the perimeter.

## License and Copyright ##
If you find any of this code useful and use it in your software or take parts of it to base your own work on then please give credit and respect the licence.

It is licensed under the terms of LGPL 3.0 and the relevant documentation for this can be found at the top of each source file and in the LICENSE text file.

The code is the work of me (Duncan Crutchley) (<dac1976github@outlook.com>).

Copyright (C) 2020 onwards Duncan Crutchley.
