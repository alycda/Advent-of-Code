# Rust

Inspired by [Chris Biscardi](https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust)

## Setup

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/Pluto-tv/AdventOfCode/). Then make sure you click on the green "Open Workspace" button within the [workspace](../../AdventOfCode.code-workspace) file. When you open a terminal, make sure you select "2024/rust" to set the CWD. Don't forget to add your ENV variable(s).

- Install [Rust](https://www.rust-lang.org/tools/install) (Homebrew not recommended, but may work ok for this repo)
  - if running in Codespaces, rust is already installed for you.
    - you may need to restart rust-analyzer after you create your first day
- Switch to Nightly:
  - `rustup update nightly`
  - `rustup override set nightly`
    - `rustup override unset` if you want to undo. see https://rust-lang.github.io/rustup/overrides.html for more info
- `brew install just tracy`
  - Github Codespaces: `cargo install just`
- `cargo install cargo-nextest cargo-generate flamegraph`

### Other

- [Shuttlings](https://www.shuttle.dev/cch)
