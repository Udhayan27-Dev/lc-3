# LC-3 Virtual Machine

A Rust implementation of the [LC-3 (Little Computer 3)](https://en.wikipedia.org/wiki/LC-3) virtual machine. Load compiled `.obj` bytecode files and run LC-3 programs with keyboard I/O, terminal output, and standard trap routines.

## Features
`
- Full fetch–decode–execute cycle with 64 KiB addressable memory
- 8 general-purpose registers (`R0`–`R7`), program counter, and condition codes (N, Z, P)
- LC-3 instruction set: ADD, AND, NOT, BR, JMP, JSR, LD, LDI, LDR, LEA, ST, STI, STR, TRAP
- Memory-mapped keyboard I/O (`KBSR` at `0xFE00`, `KBDR` at `0xFE02`)
- TRAP routines for character I/O, string output, and halting
- Raw terminal mode for interactive programs (e.g. games)

## Requirements

- [Rust](https://rustup.rs/) (2024 edition)
- A real terminal (TTY) — the VM configures stdin for raw/cooked I/O via `termios`

## Build

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <path-to-program.obj>
```

Or with the release binary:

```bash
./target/release/vm examples/hello-world.obj
```

The program path is the only required argument.

## Object File Format

Programs are loaded from binary `.obj` files:

1. **First word** — base load address (big-endian `u16`)
2. **Remaining words** — instructions loaded sequentially from that address

The program counter starts at `0x3000` by default (see `PC_START` in the register module). Ensure your assembled program's entry point matches where the VM expects execution to begin.

## Examples

The `examples/` directory includes precompiled LC-3 programs:

| File | Description |
|------|-------------|
| `hello-world.obj` | Prints a greeting |
| `test.obj` | Basic instruction test |
| `test-trap.obj` | TRAP routine test |
| `2048.obj` | 2048 game (interactive) |
| `rogue.obj` | Rogue-like game (interactive) |

```bash
cargo run -- examples/hello-world.obj
cargo run -- examples/2048.obj
```

Interactive programs read keyboard input through memory-mapped registers or TRAP calls. Run them in a terminal, not piped from a file.

## Architecture

```
┌─────────────┐     fetch      ┌──────────┐
│   Memory    │ ◄───────────── │    PC    │
│  (u16 × N)  │                └──────────┘
└──────┬──────┘                      │
       │                             ▼
       │                      ┌──────────────┐
       └────────────────────► │  Instruction │
              read/write      │   Decoder    │
                              └──────┬───────┘
                                     │
                                     ▼
                              ┌──────────────┐
                              │  Registers   │
                              │ R0–R7, COND  │
                              └──────────────┘
```

### Memory-Mapped I/O

| Address | Register | Purpose |
|---------|----------|---------|
| `0xFE00` | KBSR | Keyboard status (bit 15 set when a key is ready) |
| `0xFE02` | KBDR | Keyboard data (ASCII value of pressed key) |

### TRAP Codes

| Code | Name | Description |
|------|------|-------------|
| `0x20` | GETC | Read one character into `R0` |
| `0x21` | OUT | Write character in `R0` to stdout |
| `0x22` | PUTS | Print null-terminated string at address in `R0` |
| `0x23` | IN | Prompt and read a character into `R0` |
| `0x24` | PUTSP | Print packed string (two chars per word) |
| `0x25` | HALT | Stop execution |

## Project Structure

```
src/
├── main.rs                 # CLI, object file loader, terminal setup
└── hardware/
    ├── mod.rs              # Execution loop
    ├── vm/mod.rs           # VM state, memory, keyboard handling
    ├── register/mod.rs     # Register file and condition flags
    └── instruction/mod.rs  # Opcode decoding and execution
```
