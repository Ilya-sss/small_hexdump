# small_hexdump

A small hexdump tool. The command-line interface is written in **Python** (`argparse`), and the formatting logic is written in **Rust** (PyO3), compiled into a Python module with [maturin](https://www.maturin.rs/).

**Version:** 0.1

```
$ small_hexdump -C /bin/ls | head -3
00000000  7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00  |.ELF............|
00000010  03 00 3e 00 01 00 00 00 d0 57 00 00 00 00 00 00  |..>......W......|
00000020  40 00 00 00 00 00 00 00 b0 f3 02 00 00 00 00 00  |@...............|
```

---

## Features (v0.1)

- **Plain mode:** hex bytes only, one chunk per line.
- **Canonical mode (`-C`):** offset, hex bytes and an ASCII column, like `hexdump -C`.
- **Configurable line width (`-k`):** choose how many bytes are shown per line (default 16).
- Non-printable bytes are shown as `.` in the ASCII column.
- Clear error message if the file cannot be read.

### Not in v0.1 yet

- Collapsing repeated lines into `*` (like the real `hexdump`).
- Reading in blocks: the whole file is loaded into memory, so very large files use a lot of RAM.
- Skipping to an offset or limiting the number of bytes.

---

## Requirements

- [Rust](https://rustup.rs/) (stable toolchain)
- [uv](https://docs.astral.sh/uv/)
- Python 3.9 or newer

---

## Install

### For development (inside the project)

```bash
uv sync
uv run maturin develop --release
uv run small_hexdump -C /bin/ls
```

### As a global command

```bash
uv tool install .
small_hexdump -C /bin/ls
```

If the command is not found, run `uv tool update-shell` once and restart the terminal.

To update after changing the code:

```bash
uv tool install . --reinstall
```

### Build a wheel

```bash
uv build --wheel
```

The wheel is created in `dist/`. It contains compiled Rust code, so it only works on the same OS, architecture and Python version it was built on.

---

## Usage

```
small_hexdump [-h] [-k K] [-C] file
```

| Option | Description |
|---|---|
| `file` | File to dump |
| `-k K` | Bytes per line (default: `16`, must be at least `1`) |
| `-C` | Canonical output: offset + hex + ASCII |
| `-h` | Show help |

### Examples

Plain hex, 16 bytes per line:

```bash
small_hexdump main
```

```
7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
03 00 3e 00 01 00 00 00 20 10 00 00 00 00 00 00
```

Canonical output:

```bash
small_hexdump -C main
```

```
00000000  7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00  |.ELF............|
00000010  03 00 3e 00 01 00 00 00 20 10 00 00 00 00 00 00  |........ .......|
```

8 bytes per line:

```bash
small_hexdump -C -k 8 main
```

Only the first lines (pipe to `head`):

```bash
small_hexdump -C main | head
```

---

## How it works

The project is split in two layers. Python talks to the user, Rust does the work.

```
 small_hexdump -C -k 8 file
          │
          ▼
┌──────────────────────────┐
│  Python  (cli.py)        │  1. parse arguments with argparse
│                          │  2. read the file as bytes
│                          │  3. choose plain or canonical mode
└────────────┬─────────────┘
             │  bytes + k
             ▼
┌──────────────────────────┐
│  Rust  (src/lib.rs)      │  4. split data into chunks of k bytes
│  compiled to _core.so    │  5. format offset / hex / ASCII
│                          │  6. return one String
└────────────┬─────────────┘
             │  String
             ▼
┌──────────────────────────┐
│  Python                  │  7. print the result
└──────────────────────────┘
```

### Rust side

`src/lib.rs` exposes two functions to Python through PyO3:

- `hexdump_plain(data: &[u8], k: usize) -> String`
- `hexdump_canonical(data: &[u8], k: usize) -> String`

Both use `data.chunks(k)` to split the bytes into lines. Python `bytes` is converted to `&[u8]` automatically, and the returned `String` becomes a Python `str`. Rust builds the text and returns it, and Python does the printing.

### Python side

`cli.py` parses the arguments, reads the file, and calls one of the two Rust functions. `__init__.py` re-exports the functions so they can also be used as a library:

```python
from small_hexdump import hexdump_canonical

print(hexdump_canonical(b"Hello, world!", 16), end="")
```

### How the two languages are connected

maturin builds the Rust code as a shared library and places it inside the Python package as `small_hexdump/_core`. The same name must appear in several places:

| Where | Value |
|---|---|
| `src/lib.rs` | `mod _core { ... }` |
| `Cargo.toml` | `[lib] name = "_core"` |
| `pyproject.toml` | `module-name = "small_hexdump._core"` |
| `python/small_hexdump/__init__.py` | `from ._core import ...` |
| `python/small_hexdump/cli.py` | `from ._core import ...` |
| `pyproject.toml` | `[project.scripts]` `small_hexdump = "small_hexdump.cli:main"` |

If one of them does not match, you get an `ImportError`.

---

## Project structure

```
small_hexdump/
├── Cargo.toml                   # Rust crate, builds the cdylib "_core"
├── pyproject.toml               # Python package + maturin config + command name
├── uv.lock
├── src/
│   └── lib.rs                   # Rust logic (mod _core)
└── python/
    └── small_hexdump/
        ├── __init__.py          # re-exports the Rust functions
        └── cli.py               # argparse, calls Rust
```

---

## Development

| Task | Command |
|---|---|
| Rebuild after editing Rust | `uv run maturin develop --release` |
| Run the tool | `uv run small_hexdump -C file` |
| Check Rust errors quickly | `cargo check` |
| Install the global command | `uv tool install . --reinstall` |

Editing only Python files (`cli.py`, `__init__.py`) needs no rebuild, because the install is editable.

### Comparing with the system `hexdump`

The canonical output should match `hexdump -C`, except for repeated lines (`*`) and the final total-size line:

```bash
diff <(small_hexdump -C file) <(hexdump -C file)
```

---

## Known issues

- Piping into a program that exits early (`| head`) can print a `BrokenPipeError` traceback. It is harmless. It can be fixed by catching the error in `main()` or with `signal.signal(signal.SIGPIPE, signal.SIG_DFL)` (Linux/macOS only).
- Files are read completely into memory before printing.

---

## Roadmap

- **v0.2:** collapse repeated lines with `*`, print the final offset line.
- **v0.3:** read in blocks so large files work with low memory use.
- **Later:** `-s` (skip bytes), `-n` (limit bytes), tests with `pytest`.