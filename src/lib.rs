// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-printer-queue-management: printer queue management component model.
//!
//! Real implementation: queues, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue {
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
pub enum PrinterQueueManagementError {
    EmptyQueueName,
    DuplicateQueue(String),
    EmptyEventSubject,
}

impl fmt::Display for PrinterQueueManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueueName => write!(f, "queue name must not be empty"),
            Self::DuplicateQueue(id) => write!(f, "duplicate queue {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PrinterQueueManagementError {}

pub struct PrinterQueueManagement {
    pub name: String,
    queues: BTreeMap<String, Queue>,
    events: BTreeMap<String, Event>,
}

impl PrinterQueueManagement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            queues: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_queue(&mut self, item: Queue) -> Result<(), PrinterQueueManagementError> {
        if item.name.trim().is_empty() {
            return Err(PrinterQueueManagementError::EmptyQueueName);
        }
        let id = item.id.clone();
        if self.queues.contains_key(&id) {
            return Err(PrinterQueueManagementError::DuplicateQueue(id.clone()));
        }
        self.queues.insert(id, item);
        Ok(())
    }

    pub fn queue_count(&self) -> usize {
        self.queues.len()
    }

    pub fn get_queue(&self, id: &str) -> Option<&Queue> {
        self.queues.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PrinterQueueManagementError> {
        if event.subject.trim().is_empty() {
            return Err(PrinterQueueManagementError::EmptyEventSubject);
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

    fn sample_queue(id: &str, name: &str) -> Queue {
        Queue {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = PrinterQueueManagement::new("PrinterQueueManagement");
        assert_eq!(m.name, "PrinterQueueManagement");
        assert_eq!(m.queue_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_queue() {
        let mut m = PrinterQueueManagement::new("PrinterQueueManagement");
        m.add_queue(sample_queue("a1", "Alpha")).unwrap();
        assert_eq!(m.queue_count(), 1);
        assert!(m.get_queue("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = PrinterQueueManagement::new("PrinterQueueManagement");
        assert!(m.add_queue(sample_queue("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = PrinterQueueManagement::new("PrinterQueueManagement");
        m.add_queue(sample_queue("x", "One")).unwrap();
        assert!(m.add_queue(sample_queue("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = PrinterQueueManagement::new("PrinterQueueManagement");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
