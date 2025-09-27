
# Pulse

[![MIT License](https://img.shields.io/github/license/dayvster/pulse)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/pulse-cli?label=crates.io)](https://crates.io/crates/pulse-cli)
[![Docs.rs](https://img.shields.io/docsrs/pulse-cli?label=docs.rs)](https://docs.rs/pulse-cli)
[![GitHub issues](https://img.shields.io/github/issues/dayvster/pulse)](https://github.com/dayvster/pulse/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/dayvster/pulse)](https://github.com/dayvster/pulse/pulls)
[![GitHub stars](https://img.shields.io/github/stars/dayvster/pulse?style=social)](https://github.com/dayvster/pulse/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/dayvster/pulse?style=social)](https://github.com/dayvster/pulse/network/members)
[![Last commit](https://img.shields.io/github/last-commit/dayvster/pulse)](https://github.com/dayvster/pulse/commits/dev)
[![Repo size](https://img.shields.io/github/repo-size/dayvster/pulse)](https://github.com/dayvster/pulse)

---

**Pulse** is a modern, blazing-fast, and highly customizable command-line process monitor written in Rust. It provides real-time, color-coded stats for your system's processes, including CPU, memory, and optional IO usage. Pulse is designed for developers, sysadmins, and power users who want a beautiful, robust, and scriptable alternative to `top` and `htop`.

---

## 🚀 Features

- **Real-time, interactive process table** with color-coded CPU, RAM, and optional IO stats
- **Sort and limit**: Sort by PID, name, CPU, RAM, or RAM%, ascending or descending, and limit the number of displayed processes
- **Flexible filtering**: Track by PID, process name, all, or current user
- **Scriptable**: Use `--no-interactive` for single-shot output (great for scripts)
- **Cross-platform**: Linux and macOS supported
- **Fast and robust**: Built with Rust, handles panics gracefully
- **MIT Licensed**: Free and open source

---

## 📦 Installation

**With Cargo:**

```bash
git clone https://github.com/dayvster/pulse.git
cd pulse
cargo build --release
sudo cp target/release/pulse /usr/local/bin
```

---

## 🖥️ Usage

Monitor Firefox by name, updating every 0.5s:

```bash
pulse -n firefox -i 0.5
```

Monitor a process by PID:

```bash
pulse -p 1234
```

Show top 10 processes by CPU, descending:

```bash
pulse --sortby cpu --order desc --limit 10
```

Show IO stats as well:

```bash
pulse --io
```

Non-interactive (single-shot, for scripts):

```bash
pulse --no-interactive --sortby ram --limit 5
```

---

## ⚙️ Options

```bash
Usage: pulse [OPTIONS]

Options:
  -p, --pid <PID>            The process ID to track (can be repeated)
  -n, --name <NAME>          The process name to track (can be repeated)
  -i, --interval <INTERVAL>  Update interval in seconds [default: 1.0]
  --no-interactive           Print once and exit (for scripts)
  --sortby <SORTBY>          Sort by: pid, name, cpu, ram, rampercent [default: pid]
  --order <ORDER>            Sort order: asc, desc [default: asc]
  --limit <LIMIT>            Limit number of displayed processes [default: 30]
  --io                       Show IO (disk read/write) column
  -A, --all                  Show all processes
  -a, --current-user         Show only current user's processes
  -h, --help                 Print help
  -V, --version              Print version
```

---

## 🤝 Contributing

Contributions are welcome! Please fork the repo, create a feature branch, and submit a pull request. For bugs, ideas, or questions, open an [issue](https://github.com/dayvster/pulse/issues).

---

## 📄 License

Pulse is licensed under the MIT License. See [LICENSE](LICENSE) for details.
