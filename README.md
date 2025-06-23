# standsit

A simple Windows desktop application that reminds you to alternate between sitting and standing at customizable intervals throughout your workday.

---

## Features

- Customizable schedule via a JSON file
- Windows toast notifications with sound
- Alternates between "sit" and "stand" reminders
- Easy to configure and use

---

## Getting Started

### 1. Download or Build

**Option A: Download Executable**
- Download the latest release from your release page (add your link).
- Place the `standsit.exe` and `schedule.json` in the same folder.

**Option B: Build from Source**
- [Install Rust](https://rustup.rs/)
- Clone this repository:

```bash
git clone https://github.com/SaxsCode/standsit.git
cd standsit
```
- Build the executable:
```bash
cargo build --release
```

- The executable will be in `target/release/standsit.exe`.

### 2. Configure Your Schedule

Create or edit a `schedule.json` file in the same folder as the executable.

**Example:**
```JSON
[
{ "interval": 30, "start": "09:00", "end": "12:00" },
{ "interval": 60, "start": "13:00", "end": "17:00" }
]
```

- `interval`: Minutes between reminders
- `start`/`end`: Work block times in 24-hour format

### 3. Running the App

- **Recommended:** Open Command Prompt, navigate to the folder, and run:

```bash
standsit.exe
```

- **Alternatively:** Double-click the `.exe`

---

## How It Works

- During each scheduled block, a notification will remind you to "sit" or "stand" at the specified interval.
- The message alternates automatically.
- Outside of scheduled blocks, the app waits for the next block to start.

---

## Troubleshooting

- **No notifications?** Make sure notifications are enabled for apps on your system.
- **Errors or crashes?** Run from Command Prompt to see error messages.
- **Missing schedule file?** The app expects `schedule.json` in the same folder as the `.exe`.

---

## Contributing

1. Fork this repo
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes
4. Push and open a pull request

---

## License

MIT License. See LICENSE.

