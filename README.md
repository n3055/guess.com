# ♟ guess.com — Real-Time Multiplayer Chess

A modern, full-stack multiplayer chess web application built in **Rust** using the **[Topcoat](https://github.com/tokio-rs/topcoat)** framework, **HTMX**, and **WebSockets**. Features a dark UI inspired by Chess.com, real-time board updates, drag-and-drop gameplay, and a custom "Mad Scientist" pocket variant.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Topcoat](https://img.shields.io/badge/Topcoat-000000?style=for-the-badge&logo=tokio&logoColor=white)
![HTMX](https://img.shields.io/badge/HTMX-36C?style=for-the-badge&logo=htmx&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/Tailwind_CSS-38BDF8?style=for-the-badge&logo=tailwind-css&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)

---

## ✨ Features

- ⚡ **Real-Time Multiplayer** — Instant board sync across players via WebSocket channel broadcasts.
- 🎯 **Quick Matchmaking** — Queue system for pairing online players automatically into standard or custom games.
- 🎨 **Chess.com Inspired UI** — Modern dark theme (`#262522`) with custom board styling and smooth animations.
- ♟ **Dual Interaction Modes** — Supports both HTML5 **drag-and-drop** and **click-to-move** piece interactions.
- 📖 **Full Rule Compliance** — Complete FIDE rule validation (castling, en-passant, promotion prompt, checkmate, stalemate).
- 🧪 **Mad Scientist Variant** — A unique pocket chess mode where captured pieces return to their owner's pocket and can be dropped onto any empty square.
- 🔍 **Spectator Mode** — Watch ongoing live matches in real time.

---

## 🏗 Architecture & Code Structure

The project follows a clean modular design with separated concerns:

```
src/
├── main.rs              # Application entry point & Tokio runtime setup
├── chess_game.rs        # Core game logic, state management, & unit tests
├── ws.rs                # Tokio WebSocket server for real-time broadcasts
├── helpers.rs           # Player session cookie management
├── routes/
│   ├── mod.rs           # Route module exports
│   ├── lobby.rs         # GET / (Lobby), POST /game/create, POST /game/join
│   ├── game.rs          # GET /game/{id}, GET /game/{id}/board, POST /game/{id}/move
│   └── matchmaking.rs   # Matchmaking queue (join, status polling, cancel)
├── views/
│   ├── mod.rs           # View module exports
│   ├── layout.rs        # Root HTML layout shell & global CSS
│   ├── board.rs         # Game board & player info bar rendering
│   └── pieces.rs        # Piece SVG mapping & captured piece counter
└── js/
    └── chess_client.js  # Compiled-in JS for WS connection & drag/drop handlers
```

---

## 🎮 Game Modes

### Standard Chess
Traditional chess rules powered by the [`chess`](https://crates.io/crates/chess) crate engine.

### 🧪 Mad Scientist Chess
A fast-paced custom variant featuring pocket piece drops:
1. **Pocket Storage**: When a piece is captured, it is placed into the *owner's pocket* (retaining its original color).
2. **Piece Drop**: On your turn, instead of moving a board piece, you can drop any piece from your pocket onto an empty square.
3. **Drop Restrictions**: Pawns cannot be dropped onto the 1st or 8th ranks.

---

## 🛠 Tech Stack

| Component | Technology | Description |
|-----------|------------|-------------|
| **Framework** | [Topcoat](https://github.com/tokio-rs/topcoat) | Full-stack Rust web framework |
| **Engine** | [`chess`](https://crates.io/crates/chess) | FIDE-compliant move generation & board evaluation |
| **WebSockets** | `tokio-tungstenite` | Low-latency real-time refresh signaling |
| **Frontend** | HTMX + Tailwind CSS | Server-driven UI updates with zero heavy JS bundles |
| **Assets** | Lichess (cburnett) | High-quality SVG chess piece assets |

---

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition or newer)

### Installation & Run

```bash
# Clone the repository
git clone https://github.com/n3055/guess.com.git
cd guess.com

# Build and start the app
cargo run
```

Access the application in your browser at **`http://localhost:3000`** (WebSocket runs on port `3001`).

> **Tip**: Open two browser tabs (or one standard + one incognito tab) to test live multiplayer gameplay locally!

### Running Unit Tests

```bash
cargo test
```

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
