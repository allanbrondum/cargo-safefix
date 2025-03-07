Like `cargo clippy --fix` (or `cargo fix`), but fix only issues that cannot be a sign of code errors. 

The automatic fixes for a number of clippy warnings can silence potential coding errors, e.g. unused variables. 
In many cases the warnings are a symptom of a deeper error that cannot be automatically fixed.
The purpose of this tool is to fix warnings without the need to inspect the changes performed by the tool.
This is done by restricting the list of compiler and clippy warnings that are fixed. The warnings fixed
are listed in [fix.txt](src/fix.txt).

Install with
```
cargo install --locked --git https://github.com/allanbrondum/cargo-safefix.git
```

