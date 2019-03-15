# Turing machine in Rust

## How to use

The file `rules.txt` contains the machine's rule in the format `(initial_state,read,final_state,write,direction)`.
The rules are separated by `\n`. The first line of the file must contain the null character, the left and the right
direction characters and the exit state. For example, a valid first line would be `-,<,>,2`.

The file `tape.txt` contains the status of the tape, so, for example, `0|1|9|-`.
