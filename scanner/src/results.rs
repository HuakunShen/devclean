use prettytable::{color, Attr, Cell, Row, Table};
use std::path::{Display, PathBuf};
use human_bytes::human_bytes;

#[derive(Debug)]
pub struct AnalyzeTarget {
    path: PathBuf,
    size: u64,
    depth: u16,
}

impl AnalyzeTarget {
    pub fn new(path: PathBuf, size: u64, depth: u16) -> Self {
        AnalyzeTarget { path, size, depth }
    }
}

#[derive(Debug)]
pub struct AnalyzeTargets(pub Vec<AnalyzeTarget>);

impl From<&AnalyzeTarget> for Row {
    fn from(target: &AnalyzeTarget) -> Self {
        Row::new(vec![
            Cell::new(&target.path.to_string_lossy()).with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(human_bytes(target.size as f64).as_str()),
            Cell::new(&target.depth.to_string()),
        ])
    }
}

impl From<&AnalyzeTargets> for Table {
    fn from(targets: &AnalyzeTargets) -> Self {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Path").with_style(Attr::Bold),
            Cell::new("Size").with_style(Attr::Bold),
            Cell::new("Depth").with_style(Attr::Bold),
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
    pub fn sort_by_size(&mut self) -> &mut Self {
        self.0.sort_by(|a, b| b.size.cmp(&a.size));
        self
    }
}

impl std::fmt::Display for AnalyzeTargets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::from(self);
        write!(f, "{}", table)
    }
}
