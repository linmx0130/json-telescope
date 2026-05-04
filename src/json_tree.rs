use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Key(String),
    Index(usize),
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Object { len: usize, expanded: bool },
    Array { len: usize, expanded: bool },
    Primitive(Value),
}

#[derive(Debug, Clone)]
pub struct VisibleNode {
    pub path: Vec<PathSegment>,
    pub depth: usize,
    pub kind: NodeKind,
}

pub struct TreeState {
    pub root: Value,
    pub expanded_paths: HashSet<Vec<PathSegment>>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub wrap: bool,
}

impl TreeState {
    pub fn new(root: Value) -> Self {
        let mut expanded = HashSet::new();
        expanded.insert(Vec::new());
        Self {
            root,
            expanded_paths: expanded,
            selected: 0,
            scroll_offset: 0,
            wrap: false,
        }
    }

    pub fn flatten(&self) -> Vec<VisibleNode> {
        let mut result = Vec::new();
        self.flatten_value(&self.root, Vec::new(), 0, &mut result);
        result
    }

    fn flatten_value(
        &self,
        value: &Value,
        path: Vec<PathSegment>,
        depth: usize,
        result: &mut Vec<VisibleNode>,
    ) {
        match value {
            Value::Object(map) => {
                let expanded = self.expanded_paths.contains(&path);
                result.push(VisibleNode {
                    path: path.clone(),
                    depth,
                    kind: NodeKind::Object {
                        len: map.len(),
                        expanded,
                    },
                });
                if expanded {
                    for (key, val) in map {
                        let mut child_path = path.clone();
                        child_path.push(PathSegment::Key(key.clone()));
                        self.flatten_value(val, child_path, depth + 1, result);
                    }
                }
            }
            Value::Array(arr) => {
                let expanded = self.expanded_paths.contains(&path);
                result.push(VisibleNode {
                    path: path.clone(),
                    depth,
                    kind: NodeKind::Array {
                        len: arr.len(),
                        expanded,
                    },
                });
                if expanded {
                    for (i, val) in arr.iter().enumerate() {
                        let mut child_path = path.clone();
                        child_path.push(PathSegment::Index(i));
                        self.flatten_value(val, child_path, depth + 1, result);
                    }
                }
            }
            other => {
                result.push(VisibleNode {
                    path,
                    depth,
                    kind: NodeKind::Primitive(other.clone()),
                });
            }
        }
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn down(&mut self, visible_len: usize) {
        if self.selected + 1 < visible_len {
            self.selected += 1;
        }
    }

    pub fn expand(&mut self, visible: &[VisibleNode]) {
        if let Some(node) = visible.get(self.selected) {
            if matches!(node.kind, NodeKind::Object { .. } | NodeKind::Array { .. }) {
                self.expanded_paths.insert(node.path.clone());
            }
        }
    }

    pub fn collapse(&mut self, visible: &[VisibleNode]) {
        if let Some(node) = visible.get(self.selected) {
            self.expanded_paths.remove(&node.path);
        }
    }

    pub fn ensure_visible(&mut self, viewport_height: usize) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.selected.saturating_sub(viewport_height - 1);
        }
    }

    pub fn clamp_selected(&mut self, visible_len: usize) {
        if visible_len == 0 {
            self.selected = 0;
        } else if self.selected >= visible_len {
            self.selected = visible_len - 1;
        }
    }

    pub fn toggle_wrap(&mut self) {
        self.wrap = !self.wrap;
    }
}
