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

## 🔧 Current Functionality

### Torrent Processing

* Extracts torrent metadata including announce URLs
* Generates info hashes for torrent identification
* Based on `lava_torrent` crate for reliable parsing

### Configuration

* YAML-based configuration system
* Configurable torrent file paths
* Modular architecture for easy extension

---
