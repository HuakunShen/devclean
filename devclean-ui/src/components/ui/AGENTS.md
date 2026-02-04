# UI Components

**Purpose:** shadcn/ui component library for devclean GUI.

## STRUCTURE

All components follow shadcn/ui pattern:
- One component per file
- Radix UI primitives + Tailwind styling
- `class-variance-authority` for variant management
- `cn()` utility for class merging

## COMPONENTS

| Component | Source | Purpose |
|-----------|--------|---------|
| `button.tsx` | Radix Slot | Primary actions |
| `table.tsx` | Radix | Results display |
| `dialog.tsx` | Radix | Modals |
| `sheet.tsx` | Radix | Side panels |
| `toast.tsx` | Radix | Notifications |
| `progress.tsx` | Radix | Scan progress |
| `checkbox.tsx` | Radix | Selection |
| `dropdown-menu.tsx` | Radix | Actions menu |
| `input.tsx` | Native | Text input |
| `label.tsx` | Radix | Form labels |
| `separator.tsx` | Radix | Dividers |
| `scroll-area.tsx` | Radix | Scrollable content |
| `badge.tsx` | - | Status indicators |
| `alert.tsx` | - | Warning messages |
| `tabs.tsx` | Radix | Tab navigation |
| `drawer.tsx` | Vaul | Mobile drawer |

## PATTERNS

- **Styling:** Tailwind with `dark:` variants for dark mode
- **Variants:** Use `cva()` for button variants (default, destructive, outline, etc.)
- **Icons:** Import from `@radix-ui/react-icons` or `lucide-react`
- **Forward refs:** All components use `React.forwardRef()`

## ADDING COMPONENTS

Use shadcn CLI (if available) or copy existing pattern:
```bash
npx shadcn-ui@latest add button
```
