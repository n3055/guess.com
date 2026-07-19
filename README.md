# ♟ guess.com — Play Chess Online

A real-time multiplayer chess web application built with **Rust**, **Topcoat**, and **HTMX**. Features a sleek dark theme inspired by chess.com, live WebSocket updates, drag-and-drop piece movement, and a custom "Mad Scientist" chess variant.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebSocket](https://img.shields.io/badge/WebSocket-010101?style=for-the-badge&logo=socketdotio&logoColor=white)
![HTMX](https://img.shields.io/badge/HTMX-36C?style=for-the-badge&logo=htmx&logoColor=white)

## ✨ Features

- **Real-time Multiplayer** — Play against friends or random opponents with live WebSocket board updates.
- **Quick Matchmaking** — Instant pairing with other players searching for a game.
- **Drag & Drop + Click-to-Move** — Two intuitive ways to play your moves.
- **Standard Chess** — Full rule enforcement including castling, en passant, promotion, checkmate, and stalemate detection.
- **🧪 Mad Scientist Mode** — A custom variant where captured pieces return to the owner's pocket and can be dropped back onto the board.
- **Responsive UI** — Desktop and mobile-friendly dark theme with smooth animations.
- **Spectator Mode** — Watch any live game without joining.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Run Locally

```bash
# Clone the repo
git clone https://github.com/<your-username>/guess-chess.git
cd guess-chess

# Build and run
cargo run
```

The app will start on **http://localhost:3000** with the WebSocket server on **port 3001**.

Open two browser tabs (or one normal + one incognito) to play against yourself.

## 🏗 Project Structure

```
src/
├── main.rs              — Entry point, router setup
├── chess_game.rs         — Game logic, move validation, GameManager
├── ws.rs                 — WebSocket server for live updates
├── helpers.rs            — Player session cookie management
├── routes/
│   ├── lobby.rs          — Lobby page, game creation, join by code
│   ├── game.rs           — Game page, board endpoint, move execution
│   └── matchmaking.rs    — Quick matchmaking join/status/cancel
├── views/
│   ├── layout.rs         — HTML shell, sidebar, global CSS
│   ├── board.rs          — Chess board and player panel rendering
│   └── pieces.rs         — Piece image URLs, captured piece tracking
└── js/
    └── chess_client.js   — Client-side interaction (WebSocket, drag/drop, moves)
```

## 🎮 Game Variants

### Standard Chess
Classic chess with full FIDE rules. Checkmate your opponent to win.

### 🧪 Mad Scientist Chess
A custom variant with pocket mechanics:
1. **Captured pieces go back to the owner's pocket** (they keep their original color).
2. On your turn, you can **drop a piece from your pocket** onto any empty square instead of moving.
3. Pawns cannot be dropped on the 1st or 8th rank.

## 🛠 Tech Stack

| Layer | Technology |
|-------|-----------|
| **Backend** | Rust + [Topcoat](https://crates.io/crates/topcoat) framework |
| **Chess Engine** | [`chess`](https://crates.io/crates/chess) crate (move validation, board state) |
| **Frontend** | Server-rendered HTML + [HTMX](https://htmx.org) + [Tailwind CSS](https://tailwindcss.com) |
| **Real-time** | WebSocket via [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite) |
| **Piece Assets** | [Lichess cburnett SVGs](https://lichess.org) |

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
