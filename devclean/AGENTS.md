# devclean CLI

**Purpose:** Rust library + binary for scanning and cleaning development directories.

## STRUCTURE

```
devclean/
├── src/
│   ├── main.rs        # CLI entry (clap derive)
│   ├── lib.rs         # Public exports
│   ├── scanner.rs     # Parallel directory scanner
│   ├── cleaner.rs     # File removal with dry-run support
│   ├── results.rs     # AnalyzeTarget + table formatting
│   └── predicates/    # Detection traits
│       ├── languages/ # Language predicates (node, rust, git)
│       ├── general.rs
│       └── stop.rs
└── Cargo.toml
```

## KEY ABSTRACTIONS

### 4 Core Traits (predicates/)

| Trait | Purpose | Example |
|-------|---------|---------|
| `LanguagePredicate` | Detect project type | `is_in_project()` checks for package.json/Cargo.toml |
| `Removable` | Should this path be removed? | `is_removable()` checks node_modules in Node project |
| `Stop` | Stop directory traversal | `stop()` prevents scanning inside node_modules |
| `Reportable` | Should report this path? | Wrapper for different features (clean vs dirty-git) |

### Scanner

- `scan_parallel()` - Rayon-based parallel traversal
- `scan_recursive()` - Sequential with parallel size calculation
- Configurable depth limit + stop conditions
- Progress bar support via `indicatif`

## ADDING LANGUAGE SUPPORT

1. Create `src/predicates/languages/{lang}.rs`
2. Implement all 4 traits for a struct (see `node.rs` or `rust.rs`)
3. Add to `languages/mod.rs` exports
4. Register in `scanner.rs` `get_project_garbage_scanner()`

## COMMANDS

```bash
cargo build
cargo test
cargo run -- <path>              # Scan for cleanable dirs
cargo run -- <path> --dry-run    # Preview only
cargo run -- find-dirty-git <path>  # Find dirty git repos
```
