use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugPanel {
    pub panel_id: String,
    pub title: String,
    pub updated_at: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Default, Clone)]
pub struct DebugPanelStore {
    order: Vec<String>,
    panels: HashMap<String, DebugPanel>,
}

impl DebugPanelStore {
    pub fn upsert(&mut self, panel: DebugPanel) {
        if !self.panels.contains_key(&panel.panel_id) {
            self.order.push(panel.panel_id.clone());
        }
        self.panels.insert(panel.panel_id.clone(), panel);
    }

    pub fn ordered_panels_by_prefix(&self, prefix: &str) -> Vec<&DebugPanel> {
        self.order
            .iter()
            .filter_map(|panel_id| self.panels.get(panel_id))
            .filter(|panel| panel.panel_id.starts_with(prefix))
            .collect()
    }

    pub fn first_panel_id_by_prefix(&self, prefix: &str) -> Option<String> {
        self.ordered_panels_by_prefix(prefix)
            .first()
            .map(|panel| panel.panel_id.clone())
    }

    pub fn get(&self, panel_id: &str) -> Option<&DebugPanel> {
        self.panels.get(panel_id)
    }

    pub fn remove(&mut self, panel_id: &str) {
        self.panels.remove(panel_id);
        self.order.retain(|id| id != panel_id);
    }

    pub fn contains(&self, panel_id: &str) -> bool {
        self.panels.contains_key(panel_id)
    }

    pub fn is_empty_by_prefix(&self, prefix: &str) -> bool {
        self.ordered_panels_by_prefix(prefix).is_empty()
    }
}
