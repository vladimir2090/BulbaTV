# BulbaTV

Program for streaming torrent files to all devices on local network with smart disk space management.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Development Status](https://img.shields.io/badge/status-in%10development-yellow.svg)

---

## 🚀 Development Status

**Progress:** Early Development
BulbaTV is currently in active development phase.
Core torrent parsing functionality is implemented, streaming and network distribution features are being developed.
The project aims to provide seamless torrent streaming across local network devices with intelligent storage management.

---

## 📝 Description

**BulbaTV** allows you to stream torrent content across all devices in your local network without downloading the entire file to each device.

**Key features:**

* Stream torrent files directly to any device on local network
* Downloads only on the host machine with configurable storage limits
* Smart space management - downloads only what storage allows
* Containerized deployment for easy setup and management
* Real-time streaming without waiting for complete download

---

## 📋 TODO

* [x] Core: Torrent file parsing and info extraction
* [x] Configuration system via YAML
* [x] Modular code structure (torrent parsing separated)
* [ ] Streaming server implementation
* [ ] Network device discovery
* [ ] Smart storage management system
* [ ] Docker containerization
* [ ] Web interface for device control
* [ ] Multi-device synchronization
* [ ] Bandwidth management
* [ ] Complete testing and optimization

---

## 🔧 Installation

1.Install Rust (1.70 or later):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2.Clone the repository:

```bash
git clone https://github.com/vladimir2090/BulbaTV.git
cd BulbaTV
```

3.Create and configure `config.yaml`:

```yaml
path_torrent: "/path/to/file.torrent"
```

4.Build and run:

```bash
cargo run --release
```

---

## 📦 Dependencies

| Crate                                                           | Version | Purpose                |
|-----------------------------------------------------------------|---------|------------------------|
| [`tokio`](https://crates.io/crates/tokio)                       | 1.47    | Asynchronous runtime   |
| [`anyhow`](https://crates.io/crates/anyhow)                     | 1.0     | Error handling         |
| [`serde`](https://crates.io/crates/serde)                       | 1.0     | Serialization          |
| [`serde_yaml_ng`](https://crates.io/crates/serde_yaml_ng)       | 0.10    | YAML parser            |
| [`hex`](https://crates.io/crates/hex)                           | 0.4     | Hex encoding/decoding  |
| [`lava_torrent`](https://crates.io/crates/lava_torrent)         | 0.11    | Torrent file parser    |
| [`reqwest`](https://crates.io/crates/reqwest)                   | 0.12    | HTTP client            |
| [`percent-encoding`](https://crates.io/crates/percent-encoding) | 2.3  | URL encoding        |

For more details about each dependency, visit [crates.io](https://crates.io/).


---

## ⚙️ Current functionality

### Torrent parsing

* Extract metadata (announce URL, name, size, chunk sizes)
* Generate SHA-1 hash of information for identification.
* Used `lava_torrent` crate for reliable parsing.

### Configuration

* Read parameters from YAML file
* Configurable path to torrent file
* Easy extension of modular design.

---

## License

The project is distributed under the GPL-3.0 license.
