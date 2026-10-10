[English](README.md) | [Russian](README.ru.md)

# Okak Core

![Rust](https://img.shields.io/badge/rust-1.85+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## About the Project

**Okak Core** is the core of a voice assistant 
capable of executing commands.

Written in **Rust**. It is **agnostic** regarding specific technologies 
(TTS, ASR, UI)—these are integrated as separate modules.

### Philosophy

- **Core** handles only logic: command lookup, execution, and primitives.
- **Commands** are functions. Simple. Independent.
- **Primitives** provide system access via traits.
- **Modules** (Vosk, TTS, Tauri) are external; the core is unaware of them.

### Capabilities

- Command lookup based on text (Levenshtein distance handles typos and grammatical cases).
- Command execution with access to primitives.
- Testable without Vosk, Tauri, or a microphone.

## Installation

```bash
git clone https://github.com/Sharfim/OkakCore
cd okak_core
cargo build --release
