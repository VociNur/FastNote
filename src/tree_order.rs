use std::path::PathBuf;

use crate::paths::TREE_ORDER_FILE;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeItem {
    pub path: PathBuf,
    pub is_dir: bool,
}

pub fn load_order(dir: &std::path::Path) -> Vec<PathBuf> {
    let order_file = dir.join(TREE_ORDER_FILE);
    let json = std::fs::read_to_string(&order_file).unwrap_or_default();
    serde_json::from_str(&json).unwrap_or_default()
}

pub fn save_order(dir: &std::path::Path, items: &[TreeItem]) {
    let order_file = dir.join(TREE_ORDER_FILE);
    let paths: Vec<&PathBuf> = items.iter().map(|i| &i.path).collect();
    let json = serde_json::to_string_pretty(&paths).unwrap_or_default();
    std::fs::write(&order_file, json).ok();
}

pub fn sorted_entries(dir: &std::path::Path) -> Vec<TreeItem> {
    let all: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| !n.starts_with('.')) // ignore les fichiers cachés comme .fastnote_order
                .unwrap_or(false)
        })
        .collect();

    let order = load_order(dir);

    // Trie selon l'ordre sauvegardé, met les nouveaux à la fin
    let mut items: Vec<TreeItem> = order
        .iter()
        .filter(|p| all.contains(p))
        .map(|p| TreeItem {
            is_dir: p.is_dir(),
            path: p.clone(),
        })
        .collect();

    // Ajoute les fichiers pas encore dans l'ordre
    for path in &all {
        if !order.contains(path) {
            items.push(TreeItem {
                is_dir: path.is_dir(),
                path: path.clone(),
            });
        }
    }

    items
}
