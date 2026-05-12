# Kymus

A dictionary-based text compression service designed for constrained 
mesh networks. Built to solve a real problem: Meshtastic limits messages 
to 200 characters over LoRa. Kymus encodes common words as fixed 16-bit 
tokens (2 bytes) against a shared versioned codebook, increasing 
human-readable content per packet without increasing packet size.

## The Problem

LoRaWAN and mesh networks like Meshtastic/MeshCore impose tight payload 
limits: often 200 bytes or fewer per message. Standard text is 
inefficient at these scales. Kymus trades per-word byte cost for token 
lookups, delivering more language per packet.

## How It Works

- Each word in the shared codebook is assigned a fixed 16-bit token
- Sender encodes text → token stream using the codebook
- Receiver decodes token stream → text using the same codebook version
- Out-of-dictionary words fall back to an escape format (raw bytes)
- Codebook is versioned — sender and receiver must share the same version

## Example

| Input                        | Raw Bytes | Kymus Encoded |
|------------------------------|-----------|---------------|
| "meet at the bridge at noon" | 26 bytes  | ~12 bytes     |

## Intended Use Cases

- Meshtastic/MeshCore mesh messaging
- LoRaWAN uplinks
- Any constrained network with fixed payload limits

## Status

🚧 Active development

## License

Apache 2.0
