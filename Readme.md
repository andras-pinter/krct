# Krct
This is an imaginary, simple payments toy engine.
## About
The engine's one and only input parameter is the path pointing to the input file in CSV format.

To run the engine
```shell
cargo run --release -- input.csv
```
The output will be printed to stdout.

To redirect the output to a file
```shell
cargo run --release -- input.csv > output.csv
```
## Correctness
All the modules and business logic parts are well tested and covered by unit and end-to-end tests as well. Also, there
is no unsafe Rust code and correctness is also backed up by the Rust language strict, statically typed ecosystem.
## Efficiency
The input file is read and parsed line by line. All line is handled and processed by each Client's thread, so a line
processing does not block the main thread.
## Error handling
The engine is tried to be as error-prone as possible, however there could be some errors. If a transaction itself is
erroneous, for example withdrawal is greater than the total, is simply ignored.
### Krct error
There is a problem loading the CSV file. For example: invalid csv format, no such file, corrupted file, etc.
### OS error
If a thread died or a process died due to an OS scheduler or killer. In a well-working environment this should never
happen.
## Presumptions
### Dispute
My presumption was only an incoming (deposit) transaction could be disputed, so a withdrawal transaction cannot be
reversed.
### Client lock
My presumption was when a client account is locked, then no more transaction is possible.
### Error-prone
My presumption was to create an application which as error-prone as possible. Instead of logging or returning an error,
simply ignore it. For example, if an unknown transaction type arrives, rather ignore it, than stopping the application
with an error.
# Development
## Run all tests, checks
Run all checks, tests, like a CI Workflow. Benchmark is not included!
```shell
makers all
```
## Check code integrity
With cargo-make
```shell
makers check
```
With cargo
```shell
cargo check
```
## Run all tests
With cargo-make
```shell
makers test
```
With cargo
```shell
cargo test
```
## Run unit tests
With cargo-make
```shell
makers unit
```
With cargo
```shell
cargo test --lib
```
## Run end-to-end tests
With cargo-make
```shell
makers e2e
```
With cargo
```shell
cargo test --test krct
```
## Run lint
With cargo-make
```shell
makers lint
```
With cargo
```shell
cargo clippy
```
## Check coding style
With cargo-make
```shell
makers check-format
```
With cargo
```shell
cargo fmt -- --check
```
## Auto-format coding style
With cargo-make
```shell
makers format
```
With cargo
```shell
cargo fmt -- --emit files
```
