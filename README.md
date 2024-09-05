# The Rusty Bits: Embedded Rust

Following [The Rusty Bits](https://www.youtube.com/@therustybits)-channels videos about embedded rust.

Going to link videos here, which I've done so far, just to keep track

- Setup: https://www.youtube.com/watch?v=TOAynddiu5M
  - I'm using micro:bit version 1.5, so needed to check some configs from here https://github.com/nrf-rs/microbit/blob/main/.cargo/config.toml#L6
  - Also needed to add `defmt` and `defmt-rtt` crates
  - macos gdb: `brew install arm-non-eabi-gdb`
- Blinking a LED: https://www.youtube.com/watch?v=A9wvA_S6m7Y
  - Some adjustemnts needed for micro:bit v1.5
