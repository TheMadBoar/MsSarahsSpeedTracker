# MsSarahsSpeedTracker

Automated speedtest logger that runs internet speed tests every 5 minutes and saves results to CSV files.

## Prerequisites

**Download the Ookla Speedtest CLI separately:**

1. Get it from: https://www.speedtest.net/apps/cli
2. Extract the speedtest executable
3. Place it in the same folder as this program

## Usage

Run the executable. It will create a timestamped CSV file and start logging speed tests every 5 minutes.

Press `Ctrl+C` to stop and see your average speeds.

## Output Location

CSV files are saved to:

- **Linux**: `~/.local/share/send comic feet pics/output_logs/`
- **Windows**: `%APPDATA%\send comic feet pics\output_logs\`

## Building from Source

```bash
cargo build --release
```

---

**Developed for SarahEllae** • [twitch.tv/SarahEllae](https://twitch.tv/SarahEllae)

<sub><sup>Note: Sarah still needs to send comic feet pics. Cubic is acceptable.</sup></sub>