// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-labwc: labwc component model.
//!
//! Real implementation: labwc_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabwcItem {
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
pub enum LabwcError {
    EmptyLabwcItemName,
    DuplicateLabwcItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LabwcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabwcItemName => write!(f, "labwc_item name must not be empty"),
            Self::DuplicateLabwcItem(id) => write!(f, "duplicate labwc_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LabwcError {}

pub struct Labwc {
    pub name: String,
    labwc_items: BTreeMap<String, LabwcItem>,
    events: BTreeMap<String, Event>,
}

impl Labwc {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            labwc_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_labwc_item(&mut self, item: LabwcItem) -> Result<(), LabwcError> {
        if item.name.trim().is_empty() {
            return Err(LabwcError::EmptyLabwcItemName);
        }
        let id = item.id.clone();
        if self.labwc_items.contains_key(&id) {
            return Err(LabwcError::DuplicateLabwcItem(id.clone()));
        }
        self.labwc_items.insert(id, item);
        Ok(())
    }

    pub fn labwc_item_count(&self) -> usize {
        self.labwc_items.len()
    }

    pub fn get_labwc_item(&self, id: &str) -> Option<&LabwcItem> {
        self.labwc_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LabwcError> {
        if event.subject.trim().is_empty() {
            return Err(LabwcError::EmptyEventSubject);
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

    fn sample_labwc_item(id: &str, name: &str) -> LabwcItem {
        LabwcItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Labwc::new("Labwc");
        assert_eq!(m.name, "Labwc");
        assert_eq!(m.labwc_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_labwc_item() {
        let mut m = Labwc::new("Labwc");
        m.add_labwc_item(sample_labwc_item("a1", "Alpha")).unwrap();
        assert_eq!(m.labwc_item_count(), 1);
        assert!(m.get_labwc_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Labwc::new("Labwc");
        assert!(m.add_labwc_item(sample_labwc_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Labwc::new("Labwc");
        m.add_labwc_item(sample_labwc_item("x", "One")).unwrap();
        assert!(m.add_labwc_item(sample_labwc_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Labwc::new("Labwc");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
