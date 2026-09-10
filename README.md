# RVoic3

A comprehensive client-server communication platform built in Rust that supports voice, video, text messaging, and email communications. RVoic3 provides a modern, secure, and efficient solution for real-time peer-to-peer and multi-party communications.

## Why RVoic3?

RVoic3 was created to address the need for a unified communication platform that combines multiple communication modalities (voice, video, text, email) into a single, cohesive application. Built with Rust's performance and safety guarantees, RVoic3 aims to provide:

- **Unified Communication**: Single platform for voice, video, text, and email
- **Privacy-First**: Peer-to-peer architecture with optional centralized components
- **High Performance**: Built with Rust for low-latency audio/video streaming
- **Terminal-Based UI**: Lightweight interface using ratatui for resource efficiency
- **Flexible Database**: MySQL backend for persistent storage of users, messages, and call logs
- **Extensible Architecture**: Modular design allowing easy addition of new features

## Features

- **Voice Communication**: Real-time audio capture and streaming
- **Video Communication**: Webcam capture and video streaming using v4l
- **Text Messaging**: Real-time chat with message history
- **Email Integration**: Email communication capabilities (planned)
- **User Management**: User registration, authentication, and status tracking
- **Peer Management**: Add, remove, and track peer connections
- **Call Management**: Initiate, track, and log voice/video calls
- **Database Persistence**: MySQL backend for all data storage
- **Terminal UI**: Modern TUI built with ratatui and crossterm

## Project Status

⚠️ **Work in Progress** - This project is currently under active development and is not yet feature-complete. Many modules are still being implemented.

## Dependencies

### Runtime Dependencies

- **MySQL Server**: Required for database backend (version 5.7+ or 8.0+ recommended)
- **Linux**: v4l (Video4Linux) for webcam capture support
- **Webcam Device**: `/dev/video0` or specified device path for video functionality

### Rust Dependencies (from Cargo.toml)

- `ratatui` (0.29) - Terminal UI framework
- `crossterm` (0.29) - Cross-platform terminal manipulation
- `mysql` (26) - MySQL database connector with chrono support
- `serde` (1) - Serialization framework with derive feature
- `serde_json` (1) - JSON serialization
- `anyhow` (1) - Error handling
- `chrono` (0.4) - Date and time handling with serde support
- `uuid` (1) - UUID generation (v4, v7) with serde support
- `v4l` (0.14.0) - Video4Linux webcam capture

## Installation

### Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install MySQL Server**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install mysql-server

   # Fedora/RHEL
   sudo dnf install mysql-server

   # Start MySQL service
   sudo systemctl start mysql
   sudo systemctl enable mysql
   ```

3. **Install v4l development libraries** (for video support):
   ```bash
   # Ubuntu/Debian
   sudo apt install libv4l-dev v4l-utils

   # Fedora/RHEL
   sudo dnf install libv4l-devel v4l-utils
   ```

### Building the Project

```bash
# Clone the repository
git clone <repository-url>
cd RVoic3

# Build the project
cargo build --release

# The binary will be available at target/release/RVoic3
```

## Database Setup

1. **Create a MySQL database**:
   ```sql
   CREATE DATABASE rvoic3;
   ```

2. **Create a MySQL user** (optional, or use root):
   ```sql
   CREATE USER 'rvoic3_user'@'localhost' IDENTIFIED BY 'your_password';
   GRANT ALL PRIVILEGES ON rvoic3.* TO 'rvoic3_user'@'localhost';
   FLUSH PRIVILEGES;
   ```

3. **Initialize database tables**:
   The application will automatically create tables using the SQL schema in `src/db/init.sql`, or you can manually run:
   ```bash
   mysql -u root -p rvoic3 < src/db/init.sql
   ```

## Usage

### Environment Variables

You can configure RVoic3 using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DB` | Database name | `mydb` |
| `DB_USER` | Database username | `root` |
| `DB_PASS` | Database password | (empty) |
| `DB_URL` | Full database URL | (auto-generated) |
| `DB_PORT` | Database port | `3306` |
| `PORT` | Alternative port variable | `3306` |
| `DATABASE` | Alternative database name | `mydb` |
| `DATABASE_USER` | Alternative username | `root` |
| `DATABASE_PASS` | Alternative password | (empty) |
| `DATABASE_PASSWORD` | Alternative password | (empty) |
| `DATBASE_URL` | Alternative URL (typo intentional for compatibility) | (auto-generated) |
| `URL` | Alternative URL | (auto-generated) |
| `USERNAME` | Your username | (required) |
| `PASSWORD` | Your password | (required) |
| `PASWD` | Alternative password | (required) |
| `PASSWD` | Alternative password | (required) |
| `NICKNAME` | Display name | (optional) |

### Command-Line Arguments

```bash
# Show help
./RVoic3 -h
./RVoic3 --help

# Enable debug mode
./RVoic3 -d
./RVoic3 --debug

# Set credentials via command line
./RVoic3 --username=myuser --password=mypass --nickname="My Display Name"

# Set database connection
./RVoic3 --db_url="mysql://user:pass@localhost:3306/rvoic3"
./RVoic3 --database=rvoic3 --db_port=3306

# Configure video device
./RVoic3 -v(/dev/video0)
./RVoic3 --video(/dev/video0)

# Configure microphone
./RVoic3 -m(default)
./RVoic3 --mic(default)

# Configure audio output
./RVoic3 -a(default)
./RVoic3 --audio(default)

# Peer configuration
./RVoic3 --peer_list=peers.json
./RVoic3 --peer_map=mappings.json

# Enable text mode (default)
./RVoic3 --text
./RVoic3 -t
```

### Examples

#### Basic Usage

```bash
# Using environment variables
export DB_USER=rvoic3_user
export DB_PASS=mysecretpassword
export DB=rvoic3
export USERNAME=myuser
export PASSWORD=mypass
./target/release/RVoic3
```

#### Using Command-Line Arguments

```bash
./target/release/RVoic3 \
  --username=myuser \
  --password=mypass \
  --nickname="John Doe" \
  --database=rvoic3 \
  --db_user=rvoic3_user \
  --db_pass=mysecretpassword \
  --video(/dev/video0) \
  --mic(default) \
  -d
```

#### Using .env File

Create a `.env` file in the project root:

```env
DB=rvoic3
DB_USER=rvoic3_user
DB_PASS=mysecretpassword
USERNAME=myuser
PASSWORD=mypass
NICKNAME=My Display Name
```

Then run:
```bash
./target/release/RVoic3
```

## Project Structure

```
RVoic3/
├── src/
│   ├── main.rs           # Application entry point
│   ├── helper.rs         # CLI argument parsing and utilities
│   ├── db/               # Database module
│   │   ├── mod.rs
│   │   ├── rdb.rs        # Relational database operations
│   │   ├── cache_db.rs   # Cache database (planned)
│   │   ├── ingest.rs     # Data ingestion (planned)
│   │   └── init.sql      # Database schema
│   ├── Video/            # Video communication module
│   │   ├── mod.rs
│   │   └── video.rs      # Webcam capture using v4l
│   ├── Audio/            # Audio communication module
│   │   ├── mod.rs
│   │   ├── mic.rs        # Microphone capture (planned)
│   │   └── peripherals.rs# Audio peripherals (planned)
│   ├── Text/             # Text messaging module
│   │   ├── mod.rs
│   │   └── text.rs       # Chat functionality
│   ├── Network/          # Network communication module
│   │   └── mod.rs        # P2P networking (planned)
│   └── render/           # UI rendering module
│       ├── mod.rs
│       ├── Render.rs     # TUI rendering (planned)
│       ├── Input.rs      # Input handling (planned)
│       └── App.rs        # Application state
├── Cargo.toml            # Rust dependencies
├── Cargo.lock            # Dependency lock file
├── LICENSE               # GPL-3.0 license
├── .gitignore
├── .env                  # Environment variables (create this)
├── docker-compose.yml    # Docker configuration (planned)
└── Makefile              # Build automation (planned)
```

## Database Schema

The application uses the following tables:

- **users**: User accounts with authentication and status
- **peers**: Peer connections and their status
- **peer_connections**: Connection strings/IP mappings for peers
- **messages**: Chat messages between users
- **calls**: Call/session records with metadata
- **call_participants**: Multi-party call participant tracking
- **call_logs**: Historical call records with communication types

See `src/db/init.sql` for the complete schema.

## Development

### Running in Debug Mode

```bash
cargo run -- -d
```

### Running Tests

```bash
cargo test
```

### Checking for Errors

```bash
cargo check
```

### Formatting Code

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Roadmap

- [ ] Implement P2P networking layer
- [ ] Implement file transfer
- [ ] Add screen sharing
- [ ] Create mobile client
- [ ] Implement federation protocol

## Contributing

Contributions are welcome! Please read the LICENSE file for terms of contribution.

## License

This project is licensed under GPL-3.0-only - see the LICENSE file for details.

## Troubleshooting

### MySQL Connection Issues

If you encounter connection errors:
1. Verify MySQL is running: `sudo systemctl status mysql`
2. Check credentials in environment variables
3. Ensure the database exists: `mysql -u root -p -e "SHOW DATABASES;"`
4. Check MySQL user permissions

### Webcam Access Issues

If video capture fails:
1. Verify webcam device: `ls -l /dev/video*`
2. Check permissions: `sudo usermod -a -G video $USER`
3. Test with v4l tools: `v4l2-ctl --list-devices`

### Build Errors

If build fails:
1. Ensure Rust is up to date: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Rebuild: `cargo build`

## Support

For issues, questions, or contributions, please refer to the project repository.

