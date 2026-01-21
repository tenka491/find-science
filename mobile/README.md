# 📱 Flutter Mobile App (WebView Client)

This Flutter app loads the Rust server pages inside a WebView.
It is intended to be used alongside the Rust server in the **same project repository**.

---

## ✅ Prerequisites

Make sure you have the following installed:

### **1. Flutter**

```bash
flutter --version
```

Minimum supported version: **Flutter 3.38.7 (stable)**

### **2. Xcode (for iOS Simulator)**

* Xcode 26.x
* iOS Simulator installed

### **3. Rust Server Running**

You **must** start the Rust server before launching this app.

> **Important:**
> If the Rust server is not running, the WebView will load blank or show an error.

---

## 🔥 Quick Start

### 1. Start the Rust server first

In the `server` folder:

```bash
cd server
cargo run
```

The server should run on:

```
http://localhost:3000
```

---

### 2. Run the Flutter app

From the `mobile` folder:

```bash
cd mobile
flutter run
```

If you want to target a specific iOS simulator device:

```bash
flutter run -d "iPhone 16e"
```

---

## ⚙️ Notes

### WebView URL

The app loads the base URL defined in `config.dart`.

By default, it points to:

```dart
const String baseUrl = "http://localhost:3000";
```

---

## 🧩 Common Issues

### ❌ WebView shows blank

* Confirm Rust server is running
* Confirm URL is reachable in Safari/Chrome
* Make sure you are using `localhost` (not `127.0.0.1` or LAN IP)
