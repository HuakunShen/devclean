# devclean-ui

**Purpose:** Tauri-based desktop GUI for devclean.

## STACK

- **Framework:** Tauri v1 + React 18 + TypeScript
- **UI:** shadcn/ui components (Radix UI primitives)
- **Styling:** Tailwind CSS + dark mode
- **State:** React hooks (useState, useEffect)
- **Icons:** Radix UI icons + Lucide React

## STRUCTURE

```
devclean-ui/
├── src/
│   ├── main.tsx                 # React entry
│   ├── App.tsx                  # Theme provider + Toaster
│   ├── components/
│   │   ├── pages/home.tsx       # Main scan UI
│   │   ├── table/display-table.tsx  # Results table
│   │   ├── ui/                  # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── table.tsx
│   │   │   └── ... (18 components)
│   │   └── theme-provider.tsx
│   └── lib/
│       ├── command.ts           # Tauri invoke wrappers
│       └── model.ts             # TypeScript types (zod)
├── src-tauri/
│   └── src/main.rs              # Tauri commands
└── package.json
```

## TAURI COMMANDS

| Command | Args | Returns |
|---------|------|---------|
| `scan` | path: string, depth: number | AnalyzeTarget[] |
| `delete_dir` | path: string | void |
| `path_exists` | path: string | boolean |

## PATTERNS

- **Path alias:** `@/` maps to `src/`
- **Components:** One component per file in `components/ui/`
- **Types:** Zod schemas for runtime validation (see `model.ts`)
- **Tauri:** All commands are async, use `invoke()` from `@tauri-apps/api`

## COMMANDS

```bash
npm install
npm run dev              # Vite dev server
npm run tauri dev        # Tauri dev with hot reload
npm run tauri build      # Production build
```
