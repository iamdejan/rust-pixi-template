# Debugging

In this repository, we don't have Rust installed in local machine. Instead, `rustc` and `cargo` executables are managed by Pixi, which is why in [pixi.toml](../../pixi.toml) under `[dependencies]` section you will have `rust` listed as dependency.

The consequence is that every `rustc` and `cargo` command you want to run should be run under `pixi run` command. I will list some of the examples here:
- Check Rust version: Instead of `rustc --version`, you should run `pixi run rustc --version`.
- Run all unit tests: Instead of `cargo test`, you should run `pixi run cargo test`.
  - In the case of running all unit tests, you should follow the rule under [code_validation.md](./code_validation.md).
- Run the program: Instead of `cargo run`, you should run `pixi run cargo run`.
  - In the case of running the program (or the main function), you should execute `pixi run start` instead, following the task defined in [pixi.toml](../../pixi.toml).
