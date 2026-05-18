# Kymus

Dictionary-based text compression for constrained mesh networks.

Meshtastic caps messages at 200 bytes over LoRa. Kymus encodes common English words as fixed 16-bit tokens (2 bytes each) against a shared codebook, so you fit more language into the same packet without touching the radio layer.

## Why

LoRa and mesh networks like Meshtastic/MeshCore impose tight payload limits — often 200 bytes or fewer per message. A typical English word averages ~5 characters plus a space, so 6 bytes per word. Kymus replaces each dictionary word with a 2-byte token: roughly 3x compression on everyday text.

| Input | Raw UTF-8 | Kymus Encoded |
|-------|-----------|---------------|
| `meet at the bridge at noon` | 26 bytes | 12 bytes |

That means a 200-byte packet that normally holds ~33 words can carry ~100 encoded words.

## How it works

1. Both sender and receiver share the same versioned codebook (a frequency-ranked English word list baked into the binary at compile time).
2. The sender splits the message into words, looks each one up in the codebook, and emits the corresponding 16-bit token.
3. Words not in the codebook get escaped as raw UTF-8 bytes with a marker prefix (`0xFFFF` + length + raw bytes).
4. The receiver reads the token stream and reconstructs the original message using the same codebook.

No negotiation, no handshake, no extra packets. The codebook is static — if both sides are on the same version, it just works.

## Project structure

```
Kymus/
├── kymus-core/          # library crate — codec, codebook, encode/decode
│   └── src/
│       ├── lib.rs
│       ├── codebook.rs
│       ├── encoder.rs
│       └── decoder.rs
├── kymus-cli/           # binary crate — CLI tool for testing
│   └── src/
│       └── main.rs
├── codebooks/           # word list files
│   └── english-60k.txt
├── Cargo.toml           # workspace config
└── README.md
```

## Usage

### As a library

Add `kymus-core` to your `Cargo.toml`:

```toml
[dependencies]
kymus-core = { path = "../kymus-core" }
```

```rust
use kymus_core::codebook::Codebook;
use kymus_core::encoder::Encoder;

// Create a codebook (loads the built-in 60k word list)
let codebook = Codebook::new(None);

// Encode a message
let mut encoder = Encoder::new("meet at the bridge at noon");
let tokens = encoder.encode();

// Decode it back
let words = encoder.decode();
```

### From the CLI

```bash
cd Kymus
cargo run -p kymus-cli -- "meet at the bridge at noon"
```

### Running tests

```bash
cargo test
```

## Token format

- `0x0000` — reserved (null/padding)
- `0x0001` – `0xEA60` — dictionary words, assigned by frequency rank
- `0xFFFF` — escape prefix, followed by a length byte and raw UTF-8

The codebook currently uses the top 60,000 English words ranked by frequency from Google's Trillion Word Corpus. The most common words get the lowest token values.

## Intended use cases

- Meshtastic / MeshCore mesh messaging
- LoRaWAN uplinks
- Any constrained network with fixed payload limits
- Companion app or plugin for existing mesh clients

## Roadmap

- [x] Codebook with 60k word list
- [x] Encoder (text → tokens)
- [x] Decoder (tokens → text)
- [ ] Raw word escape handling
- [ ] Wire format serialization (tokens → bytes)
- [ ] CLI tool
- [ ] UniFFI bindings for Android/iOS
- [ ] Meshtastic integration

## Building

Requires Rust 1.70+.

```bash
git clone https://github.com/yourusername/Kymus.git
cd Kymus
cargo build
cargo test
```

## License

Apache 2.0
