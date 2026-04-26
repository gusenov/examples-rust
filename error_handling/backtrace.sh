# A backtrace is a list of all the functions that have been called to get to this point. 
# Backtraces in Rust work as they do in other languages: 
# The key to reading the backtrace is to start from the top and read until you see files you wrote. 
# That’s the spot where the problem originated. 
# The lines above that spot are code that your code has called; the lines below are code that called your code.

rm hello.txt
RUST_BACKTRACE=1 cargo run

# In order to get backtraces with this information, debug symbols must be enabled. 
# Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.
