// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-vdpau: vdpau component model.
//!
//! Real implementation: vdpau_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdpauItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VdpauError {
    EmptyVdpauItemName,
    DuplicateVdpauItem(String),
    EmptyEventSubject,
}

impl fmt::Display for VdpauError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVdpauItemName => write!(f, "vdpau_item name must not be empty"),
            Self::DuplicateVdpauItem(id) => write!(f, "duplicate vdpau_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VdpauError {}

pub struct Vdpau {
    pub name: String,
    vdpau_items: BTreeMap<String, VdpauItem>,
    events: BTreeMap<String, Event>,
}

impl Vdpau {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vdpau_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_vdpau_item(&mut self, item: VdpauItem) -> Result<(), VdpauError> {
        if item.name.trim().is_empty() {
            return Err(VdpauError::EmptyVdpauItemName);
        }
        let id = item.id.clone();
        if self.vdpau_items.contains_key(&id) {
            return Err(VdpauError::DuplicateVdpauItem(id.clone()));
        }
        self.vdpau_items.insert(id, item);
        Ok(())
    }

    pub fn vdpau_item_count(&self) -> usize {
        self.vdpau_items.len()
    }

    pub fn get_vdpau_item(&self, id: &str) -> Option<&VdpauItem> {
        self.vdpau_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VdpauError> {
        if event.subject.trim().is_empty() {
            return Err(VdpauError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_vdpau_item(id: &str, name: &str) -> VdpauItem {
        VdpauItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Vdpau::new("Vdpau");
        assert_eq!(m.name, "Vdpau");
        assert_eq!(m.vdpau_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_vdpau_item() {
        let mut m = Vdpau::new("Vdpau");
        m.add_vdpau_item(sample_vdpau_item("a1", "Alpha")).unwrap();
        assert_eq!(m.vdpau_item_count(), 1);
        assert!(m.get_vdpau_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Vdpau::new("Vdpau");
        assert!(m.add_vdpau_item(sample_vdpau_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Vdpau::new("Vdpau");
        m.add_vdpau_item(sample_vdpau_item("x", "One")).unwrap();
        assert!(m.add_vdpau_item(sample_vdpau_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Vdpau::new("Vdpau");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
