# 👁️ Blinko / 明眸

> **明眸善睐，坐如钟** | *Blinko watches, so you can rest.*

A lightweight, intelligent Windows desktop health reminder tool focused on protecting your eye and posture health.

**Design Philosophy: Keep it Simple, Keep it Stupid**

---

## 🌐 Languages

[English](README.md) | [中文](README.zh-CN.md)

---

## ✨ Core Features

- 🕐 **Timer Reminder**: Automatic 20-20-20 eye care rule reminders
- 👁️ **Blink Detection**: Camera-based detection to remind you to blink more
- 🪑 **Posture Detection**: Real-time posture monitoring and reminders
- 👀 **Desktop Icon**: Small eye icon displayed on Windows desktop top layer with low resource usage
- 📊 **Statistics**: Record your health habits
- 🔔 **System Tray**: Lightweight resident, non-intrusive but interactive
- 🔒 **Privacy First**: All image processing done locally, no data uploaded

---

## 🛠️ Tech Stack

**Pure Rust Implementation** - Simple, efficient, low resource usage

- **Language**: Rust 2021 Edition
- **Window System**: Windows API (native)
- **Image Processing**: OpenCV Rust (pure Rust, no Python required)
- **Async Runtime**: Tokio
- **Data Storage**: SQLite (rusqlite)
- **Configuration**: TOML + Serde

**Design Principles**:
- ✅ Pure Rust implementation, no external dependencies (except OpenCV)
- ✅ Minimal dependencies, keep it simple
- ✅ Low resource usage (memory < 50MB)
- ✅ Native Windows experience

---

## 📦 Project Structure

```
Blinko/
├── src/                    # Rust source code
│   ├── main.rs            # Main entry point
│   ├── config.rs          # Configuration management
│   ├── database.rs        # Database operations
│   ├── detection.rs       # Blink/posture detection
│   ├── reminder.rs        # Timer reminder
│   ├── tray.rs           # System tray
│   └── window.rs         # Desktop top layer window
├── docs/                   # Project documentation
├── Cargo.toml             # Rust dependencies
└── README.md              # This file
```

---

## 🚀 Quick Start

### Requirements

- **Rust** 1.70+ (recommended to install via rustup)
- **Windows** 10/11
- **OpenCV** 4.x (install via vcpkg or system package manager)

### Installing OpenCV (Windows)

#### Method 1: Using vcpkg (Recommended)

```powershell
# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Install OpenCV
.\vcpkg install opencv4:x64-windows

# Set environment variable
$env:OPENCV_DIR = "C:\path\to\vcpkg\installed\x64-windows"
```

#### Method 2: Using Pre-built Version

Download OpenCV pre-built version and set `OPENCV_DIR` environment variable to the installation directory.

### Build and Run

```bash
# Clone repository
git clone <repository-url>
cd Blinko

# Run in development mode
cargo run

# Build release version
cargo build --release

# Run release version
cargo run --release
```

### First Run

1. The program will display an icon in the system tray
2. A small eye icon (64x64 pixels) will appear on the desktop top layer
3. Default reminder interval: every 20 minutes
4. Configuration file: `%APPDATA%\blinko\config.toml`
5. Database: `%LOCALAPPDATA%\blinko\blinko.db`

---

## ⚙️ Configuration

Edit configuration file `%APPDATA%\blinko\config.toml`:

```toml
[reminder]
enabled = true
interval_minutes = 20

[blink_detection]
enabled = false
check_interval_seconds = 5
threshold_seconds = 30

[posture_detection]
enabled = false
sensitivity = "medium"
```

---

## 🎯 Feature Description

### Timer Reminder

- Default reminder interval: every 20 minutes
- Follows 20-20-20 eye care rule
- Adjustable interval via configuration

### Blink Detection

- Real-time blink frequency detection using camera
- Reminds when no blinking detected for extended period
- All processing done locally for privacy protection

### Posture Detection

- Detects head and shoulder positions
- Alerts when abnormal posture detected
- Adjustable detection sensitivity

### Desktop Icon

- Displays small eye icon on Windows desktop top layer
- 64x64 pixels, low resource usage
- Always on top, doesn't obstruct important content

---

## 📊 Performance Metrics

- **Memory Usage**: < 50MB (idle)
- **CPU Usage**: < 1% (idle), < 15% (detecting)
- **Startup Time**: < 2 seconds
- **Detection Latency**: < 500ms

---

## 🔒 Privacy Protection

- ✅ All image processing done locally
- ✅ No image data saved
- ✅ No data uploaded to servers
- ✅ Statistics stored locally only
- ✅ Fully offline operation

---

## 🛠️ Development

### Code Style

Follow Rust official code style:
```bash
cargo fmt
cargo clippy
```

### Testing

```bash
cargo test
```

---

## 📖 Documentation

Detailed documentation available in [docs/](./docs/) directory:

- [Requirements](./docs/requirements/) - Detailed feature requirements and analysis
- [Design](./docs/design/) - UI/UX design and architecture
- [Development Guide](./docs/development/) - Development standards and contribution guide

---

## 🎯 Brand Positioning

- **Chinese Name**: 明眸 - Elegant, culturally rich, focused on eye health
- **English Name**: Blinko - Derived from "blink", light and friendly
- **Logo Concept**: Half-closed owl/panda/robot with "bright eyes" shine

---

## 🤝 Contributing

Issues and Pull Requests are welcome!

### Contribution Guide

1. Fork this repository
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

---

## 📄 License

This project is licensed under MIT License - see [LICENSE](LICENSE) file for details

---

## 🙏 Acknowledgments

- OpenCV community
- Rust community
- All contributors

---

**Made with ❤️ for your eye health**

*Keep it simple, keep it stupid.*
