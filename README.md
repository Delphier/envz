# envz

[![Crates.io](https://img.shields.io/crates/v/envz.svg)](https://crates.io/crates/envz)
[![Docs.rs](https://img.shields.io/docsrs/envz)](https://docs.rs/envz)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

A small Rust library for **persistently** reading and modifying Windows
environment variables through the registry.

Unlike `std::env::set_var`, which only changes the environment of the
current process, `envz` writes directly to the `Environment` key under
`HKEY_CURRENT_USER` (or any registry key you provide), so the changes
survive process exit and are picked up by newly started programs — the
same effect as editing environment variables through the Windows GUI.

> **Platform:** Windows only. This crate depends on the `windows` and
> `windows-registry` crates and will not build on other platforms.

## Features

- Get, set, and remove persistent user environment variables.
- Support for both regular (`REG_SZ`) and expandable
  (`REG_EXPAND_SZ`) string values, so values containing references
  like `%JAVA_HOME%` are stored correctly.
- Convenient helpers for managing the `Path` variable:
  - read it as a `Vec<PathBuf>`
  - append (`push`) or prepend (`insert`) entries, skipping
    case-insensitive duplicates
  - remove entries
- Works against `HKEY_CURRENT_USER\Environment` by default, or against
  any registry key you supply (e.g. for machine-wide/system environment
  variables, or for testing).
- A `var` module with free functions for quick, one-off changes without
  having to construct an `Environment` yourself.
- Errors are unified into a single `envz::Error` / `envz::Result` type.

## Installation

Add `envz` to your `Cargo.toml`:

```toml
[dependencies]
envz = "0.1"
```

Or, to track the repository directly:

```toml
[dependencies]
envz = { git = "https://github.com/Delphier/envz" }
```

## Usage

### Reading and writing a variable

```rust
use envz::Environment;

fn main() -> envz::Result<()> {
    let env = Environment::new()?; // HKEY_CURRENT_USER\Environment

    // Set a plain value.
    env.set("MY_APP_HOME", r"C:\Program Files\MyApp")?;

    // Set an expandable value (references other variables).
    env.set_expand("MY_APP_CONFIG", format!("{}\\config", envz::var::placeholder("MY_APP_HOME")))?;

    // Read a value back.
    if let Some(value) = env.get("MY_APP_HOME")? {
        println!("MY_APP_HOME = {}", value.to_string_lossy());
    }

    // Remove a value.
    env.remove("MY_APP_HOME")?;

    Ok(())
}
```

### Quick one-off changes with the `var` module

```rust
use envz::var;

fn main() -> envz::Result<()> {
    var::set("MY_APP_HOME", r"C:\Program Files\MyApp")?;
    var::set_expand("MY_APP_CONFIG", "%MY_APP_HOME%\\config")?;
    var::remove("MY_APP_HOME")?;
    Ok(())
}
```

### Managing `Path`

```rust
use envz::var::path;

fn main() -> envz::Result<()> {
    // Append a directory to the end of PATH (skipped if already present).
    path::push(r"C:\Tools\bin")?;

    // Prepend a directory to the front of PATH.
    path::insert(r"C:\Tools\priority\bin")?;

    // Remove a directory from PATH.
    path::remove(r"C:\Tools\bin")?;

    Ok(())
}
```

### Using a custom registry key

`Environment::create` lets you target a different key, for example the
system-wide environment variables (which typically requires
administrator privileges) or a key used in tests:

```rust
use envz::Environment;
use windows_registry::LOCAL_MACHINE;

fn main() -> envz::Result<()> {
    let system_env = Environment::create(
        LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
        true, // whether Path is stored as an expandable string on this key
    )?;

    system_env.set("MY_SERVICE_HOME", r"C:\Services\MyService")?;
    Ok(())
}
```

## Notes on change propagation

`envz` writes directly to the registry, exactly like the Windows
Environment Variables dialog does. Already-running processes (including
the current process, and shells such as `cmd.exe` or PowerShell that
were already open) will **not** see the new values until they are
restarted, or until the system broadcasts a `WM_SETTINGCHANGE` message
(which the Windows GUI does automatically, but this crate does not do
for you).

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
