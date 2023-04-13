# ram_wiper
A RAM wiper for Linux in Rust.

All you need to do is to compile it and run it via a bash. 
The argument -k will kill all processes owned by the user, except for the ones that are ancestral to ram_wiper, and overwrite their memory.
