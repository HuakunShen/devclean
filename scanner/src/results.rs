use human_bytes::human_bytes;
use prettytable::{color, Attr, Cell, Row, Table};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct AnalyzeTarget {
    pub path: PathBuf,
    pub depth: u16,
    pub size: Option<u64>,
}

impl Ord for AnalyzeTarget {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

impl std::fmt::Display for AnalyzeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.path.to_string_lossy(),
            human_bytes(self.size.unwrap_or(0) as f64)
        )
    }
}

impl AnalyzeTarget {
    pub fn new(path: PathBuf, depth: u16, size: Option<u64>) -> Self {
        AnalyzeTarget { path, depth, size }
    }
}

#[derive(Debug)]
pub struct AnalyzeTargets(pub Vec<AnalyzeTarget>);

impl From<&AnalyzeTarget> for Row {
    fn from(target: &AnalyzeTarget) -> Self {
        Row::new(vec![
            Cell::new(&target.path.to_string_lossy())
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(&target.depth.to_string()),
            Cell::new(human_bytes(target.size.unwrap_or(0) as f64).as_str()),
        ])
    }
}

impl From<&AnalyzeTargets> for Table {
    fn from(targets: &AnalyzeTargets) -> Self {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Path").with_style(Attr::Bold),
            Cell::new("Depth").with_style(Attr::Bold),
            Cell::new("Size").with_style(Attr::Bold),
        ]));
        for target in &targets.0 {
            table.add_row(Row::from(target));
        }
        table
    }
}

impl AnalyzeTargets {
    pub fn to_table(&self) -> Table {
        Table::from(self)
    }
}

impl std::fmt::Display for AnalyzeTargets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::from(self);
        write!(f, "{}", table)
    }
}
