# Pulse

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Description

A command-line tool written in Rust to display live updating CPU and MEM usage of a specified PID or process name.

## Table of Contents

- [Pulse](#pulse)
  - [Description](#description)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Options](#options)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- Real-time monitoring of CPU and MEM usage for a specific process.
- Supports monitoring processes by PID or process name.
- Customizable update interval.
- Currently Supports Linux and MacOS.

## Installation

You can install this tool using `cargo`, the Rust package manager:

```bash
git clone https://github.com/dayvster/pulse.git
cd pulse
cargo build --release && cp target/release/pulse /usr/local/bin
```

## Usage

To use Pulse, simply run it from the command line with the desired PID or process name:

```bash
pulse -n firefox -i 0.5
```

The above command will monitor the CPU and MEM usage of the Firefox process with an update interval of 0.5 seconds.

```bash
pulse -p 1234
```

The above command will monitor the CPU and MEM usage of the process with PID 1234 with the default update interval of 1 second.

The tool will display real-time CPU and MEM usage information for the specified process. To exit the monitoring mode, press Ctrl + C.

## Options

```bash
Usage: pulse [OPTIONS]

Options:
  -p, --pid <PID>            The process ID of the process we wish to track.
                             EXAMPLE: 1234
                             
  -n, --name <NAME>          The name of the process to track.
                             EXAMPLE: firefox
                             
  -i, --interval <INTERVAL>  The interval in seconds between each sample.
                             EXAMPLE: 1.5 [default: 1.0]
  -h, --help                 Print help
  -V, --version              Print version
```


## Contributing

Contributions are welcome! If you'd like to contribute to the Pulse project, please follow these steps:

1) Fork the repository.
2) Create a new branch for your feature or bugfix: git checkout -b feature-name.
3) Make your changes and commit them: git commit -m 'Add some feature'.
4) Push to the branch: git push origin feature-name.
5) Create a pull request on GitHub.

## License

Pulse is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
