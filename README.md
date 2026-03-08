# Rust Pixi Template

This is a template repository for Rust-based projects, but using Pixi as some sort of version manager for Rust. See [this](https://pixi.prefix.dev/latest/tutorials/rust/) for the rationale.

## Prerequisites

Before running this program, ensure you have the following installed:

1. **Pixi** - A package manager for Rust and other languages. Install it by following the official guide at [https://pixi.prefix.dev/latest/](https://pixi.prefix.dev/latest/).

2. **Download Dependencies** - Once Pixi is installed, run the following command to download all project dependencies:
   ```bash
   pixi install
   ```

3. **Setup Pre-commit** - Install the pre-commit hooks to automatically run linting and build checks before push:
   ```bash
   pixi run pre-commit install --hook-type pre-push
   ```

   > **Note:** Re-run this command whenever `.pre-commit-config.yaml` is changed to update the hooks.

4. **Build the project** - If you want to build the project without running it, run the following command:
   ```bash
   pixi run build
   ```

## Run the Program

Use the following command to build and run the project:
   ```bash
   pixi run start
   ```

### Available Commands

| Command | Description |
|---------|-------------|
| `pixi run build` | Build the Rust project |
| `pixi run start` | Build and run the program |
| `pixi run test` | Build and run all unit tests |
| `pixi run fmt` | Format the code using rustfmt |
| `pixi run lint` | Run clippy linter (includes fmt check) |
| `pixi run lint-fix` | Auto-fix linting issues |
| `pixi run clean` | Clean build artifacts |
| `pixi run doc` | Generate documentation |
| `pixi run doc-open` | Generate and open documentation |

## Project Tree Structure

```
Root/
├── Cargo.toml              # Rust package manifest
├── Cargo.lock              # Dependency lockfile
├── pixi.toml               # Pixi workspace configuration
├── pixi.lock               # Pixi lockfile
├── .gitignore              # Git ignore rules
├── .gitattributes          # Git attributes
├── .editorconfig           # Editor configuration
├── .pre-commit-config.yaml # Pre-commit hook configuration
├── .github/                # GitHub workflows
│   └── workflows/
│       ├── pr-pipeline.yaml
│       └── trunk-pipeline.yaml
├── src/
│   └── main.rs             # Main entry point
└── .kilocode/              # Kilo Code assistant rules
    └── rules/
        ├── code_generation.md
        └── code_validation.md
```

This section explains the purpose of each file in the repository:

### Root Directory Files

| File | Description |
|------|-------------|
| [`Cargo.toml`](Cargo.toml) | Rust package manifest that defines the project metadata, dependencies, and build configuration. It specifies the package name, version, edition, and linter rules. |
| [`Cargo.lock`](Cargo.lock) | Automatically generated file that locks dependency versions to ensure reproducible builds. Commit this to Git for consistent builds across machines. |
| [`pixi.toml`](pixi.toml) | Pixi workspace configuration that defines the project name, authors, platforms, available tasks (build, test, lint, etc.), and dependencies (Rust toolchain). |
| [`pixi.lock`](pixi.lock) | Lockfile for Pixi that pins the exact versions of all dependencies. This file should be committed to ensure reproducible environments. |
| [`.gitignore`](.gitignore) | Specifies files and directories that Git should ignore (e.g., build artifacts in `target/`, IDE files, pixi environments in `.pixi/`). |
| [`.gitattributes`](.gitattributes) | Git configuration that sets `pixi.lock` to use binary merging (to avoid merge conflicts) and marks it as YAML-generated. |
| [`.editorconfig`](.editorconfig) | Editor configuration that ensures consistent coding style across different editors (e.g., 4-space indentation for Rust files). |
| [`.pre-commit-config.yaml`](.pre-commit-config.yaml) | Configuration for pre-commit hooks that run linting and build checks before commits/pushes. |

### Source Files

| File | Description |
|------|-------------|
| [`src/main.rs`](src/main.rs) | The main entry point of the Rust application. Currently contains a simple "Hello, world!" program. |

### Configuration Directories

| Path | Description |
|------|-------------|
| [`.github/`](.github/) | GitHub Actions workflows for CI/CD pipelines. Contains `pr-pipeline.yaml` for pull request checks and `trunk-pipeline.yaml` for the main branch pipeline. |
| [`.kilocode/`](.kilocode/) | Directory containing development rules and guidelines for the Kilo Code assistant. Includes code generation and validation rules. |
