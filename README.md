# rusty-mandelbrot
My implementation of the Mandelbrot set in Rust. I got the idea after seeing a buggy and non-working implementation of this algorithm in the book Rust In Action. 

The bugs in its implementation have been fixed, code refactored and reworked from scratch, and each step of the algorithm has been extensively commented. This is how it should have been in the book.

To render locally in your terminal, simply pull this repo, and then run:

```
# install cargo and rust compiler (macOS and Linux only). For Windows, download via https://win.rustup.rs/
curl https://sh.rustup.rs -sSf | sh

# run program
cargo run
```

Make your terminal full screen to see it in its entirety. If it looks weird, adjust the width and height settings inside the main.rs file.
