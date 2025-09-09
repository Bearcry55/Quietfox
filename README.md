# 🦊 Quietfox Browser

A lightweight, privacy-focused web browser built with Rust, designed for users who value their privacy and want a clean, distraction-free browsing experience.

<img width="500" height="500" alt="image" src="https://github.com/user-attachments/assets/213af324-198c-435b-a12a-c43bd62f1038" />


## 🚀 Project Status

**⚠️ This project is actively under development** - New features and improvements are being added regularly. While functional, expect frequent updates and potential breaking changes as we work toward a stable release.

## ✨ Features

### 🔒 Privacy & Security
- **No Tracking**: Automatically blocks cookies, localStorage, and sessionStorage
- **URL Cleaning**: Removes tracking parameters from URLs (utm_*, fbclid, gclid, etc.)
- **Random User Agents**: Anti-fingerprinting with rotating user agent strings
- **Clean Interface**: No ads, no distractions, just pure browsing

### 🌐 Multi-Engine Search
- **Multiple Search Engines**: Choose from Brave Search, DuckDuckGo, Startpage, or Google
- **One-Click Switching**: Easy engine selection with visual indicators
- **Privacy-First Defaults**: Starts with Brave Search for maximum privacy

### ⚡ Performance & Usability
- **Lightweight**: Built with Rust for speed and efficiency
- **Native Performance**: Uses system webview for optimal resource usage
- **Keyboard Shortcuts**: Navigate with Alt+Left (back), F5 (reload), Ctrl+/- (zoom)
- **Clean UI**: Modern glassmorphism design with smooth animations

### 🎨 User Experience
- **Responsive Design**: Beautiful gradient background with blur effects
- **Focus Mode**: Auto-focus search box for instant typing
- **Visual Feedback**: Hover effects and smooth transitions
- **Custom Icon**: Distinctive fox logo in window title bar

## 📋 Requirements

- **Rust**: Version 1.70 or higher
- **Cargo**: Rust package manager (comes with Rust)
- **Operating System**: Windows 10+, macOS 10.12+, or Linux with GTK 3.18+

## 🛠️ Installation

### Step 1: Install Rust and Cargo

If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) and follow the instructions, or run:

```bash
# On Unix-like systems (Linux, macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, download and run rustup-init.exe from rustup.rs
```

After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

### Step 2: Clone or Download the Project

Download all project files to a folder:
- `Cargo.toml`
- `main.rs` (place in `src/` folder)
- `init.js`
- `homepage.html`
- `Quietfox.png`

Your project structure should look like:
```
quietfox-browser/
├── Cargo.toml
├── src/
│   └── main.rs
├── init.js
├── homepage.html
└── Quietfox.png
```

### Step 3: Build and Run

Navigate to the project directory and compile:

```bash
cd quietfox-browser
cargo build --release
```

Run the browser:
```bash
cargo run --release
```

## 🎯 Usage

1. **Launch**: Run `cargo run` or execute the compiled binary
2. **Search**: Type your query and press Enter or click the search icon
3. **Switch Engines**: Click any search engine button to change providers
4. **Navigate**: Use keyboard shortcuts for quick navigation
5. **Privacy**: Enjoy automatic tracking protection and clean URLs

## ⌨️ Keyboard Shortcuts & Key Bindings

Quietfox Browser comes with intuitive keyboard shortcuts for efficient browsing:

### Navigation & Browsing
- `Enter` - Execute search with current query
- `Alt + Left Arrow` - Navigate back in browser history
- `F5` - Reload/refresh current page
- `Tab` - Navigate between UI elements

### Zoom Controls
- `Ctrl + =` (or `Ctrl + Plus`) - Zoom in (increase page size)
- `Ctrl + -` (or `Ctrl + Minus`) - Zoom out (decrease page size)  
- `Ctrl + 0` - Reset zoom to default (100%)

### Search Engine Switching
- Click engine buttons or use mouse to switch between:
  - **Brave Search** (default)
  - **DuckDuckGo** 
  - **Startpage**
  - **Google**

### Privacy Features (Automatic)
- **URL Cleaning** - Automatically removes tracking parameters
- **Cookie Blocking** - Prevents cookie storage and reading
- **Storage Blocking** - Disables localStorage and sessionStorage

### Pro Tips
- The search box auto-focuses when the browser starts - just start typing!
- All keyboard shortcuts work immediately without needing to click first
- Zoom levels are remembered during your browsing session
- Use `Alt + Left` to quickly go back instead of clicking browser buttons

## 🤝 Contributing

This project is in active development! Contributions are welcome:

- 🐛 **Bug Reports**: Open issues for any problems you encounter
- 💡 **Feature Requests**: Suggest new privacy or usability features
- 🔧 **Pull Requests**: Help improve code, documentation, or features
- 📖 **Documentation**: Help improve this README or add code comments

## 🗺️ Roadmap

### Current Focus
- [ ] Bookmark management system
- [ ] Download manager
- [ ] Tab support
- [ ] History management with privacy controls
- [ ] Extension system for ad blockers

### Future Plans
- [ ] Tor integration for ultimate privacy
- [ ] Built-in VPN support
- [ ] Custom DNS over HTTPS
- [ ] Password manager integration
- [ ] Dark/light theme switching

## 📝 License

This project is open source. Please check the license file for specific terms.

## ⚠️ Disclaimer

Quietfox Browser is experimental software under active development. While we implement privacy features, please use additional privacy tools for sensitive browsing. The browser is not yet suitable for production use.

## 🔗 Links

- **Issues & Bug Reports**: [GitHub Issues](#)
- **Discussions**: [GitHub Discussions](#)
- **Latest Releases**: [GitHub Releases](#)

---

*Built with 🦀 Rust and ❤️ for privacy* 
