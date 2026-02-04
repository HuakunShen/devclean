# Predicates

**Purpose:** Detection logic for identifying cleanable directories.

## FILES

```
predicates/
├── mod.rs           # Trait definitions (Removable, Reportable, Stop)
├── general.rs       # General predicates
├── stop.rs          # Stop conditions (HiddenDirStop, IsFileStop)
└── languages/       # Language-specific predicates
    ├── mod.rs       # LanguagePredicate trait
    ├── node.rs      # NodeModulesPredicate
    ├── rust.rs      # RustTargetPredicate
    └── git.rs       # GitDirtyRepoPredicate
```

## TRAIT SYSTEM

All language predicates implement 4 traits:

1. **LanguagePredicate** - `is_in_project(path)` - Detect project context
2. **Removable** - `is_removable(path)` - Should delete this?
3. **Stop** - `stop(path)` - Stop scanning deeper?
4. **Reportable** - `report(path)` - Include in results?

## ADDING A LANGUAGE

Copy `node.rs` or `rust.rs` pattern:

```rust
#[derive(Debug, Clone)]
pub struct MyLangPredicate;

impl LanguagePredicate for MyLangPredicate {
    fn is_in_project(&self, path: &Path) -> bool {
        // Check for language-specific marker file
        path.parent().map_or(false, |p| p.join("marker.file").is_file())
    }
}

impl Removable for MyLangPredicate {
    fn is_removable(&self, path: &Path) -> bool {
        self.is_in_project(path) && path.file_name() == Some("cache_dir".as_ref())
    }
}

impl Stop for MyLangPredicate {
    fn stop(&self, path: &Path) -> bool { self.is_removable(path) }
}

impl Reportable for MyLangPredicate {
    fn report(&self, path: &Path) -> bool { self.is_removable(path) }
}
```

Register in `scanner.rs`:
```rust
report_conditions: vec![
    Box::new(MyLangPredicate {}),
    // ...
]
```
