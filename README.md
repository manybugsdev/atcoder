# atcoder

## How to test

1. Register Bookmarklet

   Add bookmark and set its link as `javascript:(The content of copyAtcoderTestcases.js)`

1. Download Testcases

   When you visit the task page at Atcoder, click the bookmark. Then, the shell command is in your clipboard and run it. Finally, Testcases are the `test` directory.

1. Add your code

   Add your code to `src/bin`.

1. Cargo test

   `cargo test --lib`
