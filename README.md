[[Rust](https://github.com/nix-rs/rtop/workflows/Rust/badge.svg?event=check_run)](https://github.com/nix-rs/rtop/actions/workflows/rust.yml)


> **WARNING !!!** This package is in **prototype** and testing stage, and only working for
> **Quad core** processor on **arch linux** with **SSD**.

# rtop
rtop is a resource monitor for linux, written purely in rust. Using less ram and cpu then *htop* and *btop++*.  This is my first project written in rust. Currently this project is in pre-release stage and have full motivation to release a stable version with more features as soon as possible.

## Features

 - Cached, Available, Used Memory Monitor.
 - Usage, Speed, Temperature of all CPUs.
 - Per Process monitoring like command, memory, CPU, etc.
 - Network Upload and Download.
 - Up time, Number of Process Running, Battery Charging and Percentage Indicator.
 - I/O, Disk Usage.

**Upcoming features**
 - More temperature indicator of system.
 -  History of all processes usage.
 - Sorting of processes per memory, CPU, PPID, etc.
 - Reverse sorting.
 - Set custom refresh time
 - More theme
 - Custom color

## Installation

    cargo install --git https://github.com/nix-rs/rtop

## Screenshot
![Screenshots](/assets/ss.png)

## License
 [MIT](https://github.com/nix-rs/rtop/blob/main/LICENSE)
 
## Feedback
Please mail your feedback to aniket2contact [at] gmail [dot] com
Your feedback is appreciated !
