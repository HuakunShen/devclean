# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-04
**Project:** devclean - A tool for cleaning up development directories
**Stack:** Rust (CLI) + Tauri + React + TypeScript (GUI)

## OVERVIEW

`devclean` is a hybrid project with:
- **CLI Tool** (`devclean/`): Rust library + binary for scanning and cleaning dev directories
- **GUI App** (`devclean-ui/`): Tauri-based desktop app with React/TypeScript frontend

Core functionality: Scan directories for removable artifacts (node_modules, target/, dirty git repos) and clean them.

## STRUCTURE

```
.
├── devclean/              # Rust CLI crate
│   ├── src/
│   │   ├── main.rs        # CLI entry point (clap)
│   │   ├── lib.rs         # Library exports
│   │   ├── scanner.rs     # Directory scanning engine (rayon parallel)
│   │   ├── cleaner.rs     # File removal logic
│   │   ├── results.rs     # Result types & table formatting
│   │   └── predicates/    # Detection logic for languages/artifacts
│   │       ├── languages/ # Language-specific predicates
│   │       ├── general.rs
│   │       └── stop.rs
│   └── Cargo.toml
├── devclean-ui/           # Tauri GUI app
│   ├── src/
│   │   ├── main.tsx       # React entry
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── pages/home.tsx      # Main UI
│   │   │   ├── table/display-table.tsx
│   │   │   └── ui/        # shadcn/ui components
│   │   └── lib/
│   │       ├── command.ts # Tauri invoke wrappers
│   │       └── model.ts   # TypeScript types
│   ├── src-tauri/
│   │   └── src/main.rs    # Tauri Rust backend
│   └── package.json
└── .github/workflows/     # CI/CD
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add language support | `devclean/src/predicates/languages/` | Implement 4 traits: LanguagePredicate, Removable, Stop, Reportable |
| CLI args & commands | `devclean/src/main.rs` | Uses clap derive macros |
| Scanning logic | `devclean/src/scanner.rs` | Rayon parallel iteration |
| GUI backend commands | `devclean-ui/src-tauri/src/main.rs` | Tauri invoke handlers |
| GUI frontend | `devclean-ui/src/components/pages/home.tsx` | Main scan UI |
| Shared types | `devclean/src/results.rs` | AnalyzeTarget, used by both CLI & GUI |

## CONVENTIONS

### Rust (devclean/)
- **Traits**: Core abstraction via 4 traits in `predicates/`
  - `LanguagePredicate`: Detect if path is in a project of specific language
  - `Removable`: Check if path should be removed
  - `Stop`: Check if directory traversal should stop
  - `Reportable`: Wrapper over Removable for different features
- **Parallelism**: Uses `rayon` for parallel directory scanning
- **Error handling**: `color-eyre` for pretty error reporting
- **Naming**: `PascalCase` for predicates (e.g., `NodeModulesPredicate`)

### TypeScript/React (devclean-ui/)
- **Components**: shadcn/ui pattern - each component in `components/ui/`
- **Styling**: Tailwind CSS with dark mode support
- **State**: React hooks (useState, useEffect)
- **Icons**: Radix UI icons + Lucide
- **Path aliases**: `@/` maps to `src/`

## ANTI-PATTERNS

- **DO NOT** use blocking operations in Tauri commands (use async)
- **DO NOT** add language support without implementing all 4 traits
- **AVOID** deep recursion in scanners (use depth limits)

## COMMANDS

```bash
# CLI - Build & Test
cd devclean
cargo build
cargo test
cargo run -- <path>        # Scan for cleanable directories
cargo run -- find-dirty-git <path>

# GUI - Dev
cd devclean-ui
npm install
npm run dev                 # Vite dev server + Tauri
npm run tauri dev
npm run tauri build         # Production build
```

## NOTES

- CLI and GUI share the same core library (`devclean` crate)
- GUI uses Tauri's invoke API to call Rust functions from TypeScript
- Scanning uses `rayon` for parallel filesystem traversal
- Progress bars use `indicatif` crate in CLI
- Table formatting uses `prettytable-rs`
