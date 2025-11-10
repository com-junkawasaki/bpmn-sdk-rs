//! Event builders (start, end, intermediate).

use crate::core::elements::{StartEvent, EndEvent, IntermediateEvent, EventDefinition};
use crate::core::types::ElementId;

/// Start event builder
pub struct StartEventBuilder {
    id: ElementId,
    name: Option<String>,
    event_definition: Option<EventDefinition>,
}

impl StartEventBuilder {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            name: None,
            event_definition: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn message(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Message {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn timer(mut self, expression: impl Into<String>) -> Self {
        self.event_definition = Some(EventDefinition::Timer {
            expression: expression.into(),
        });
        self
    }

    pub fn signal(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Signal {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn error(mut self, error_code: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Error {
            error_code: error_code.map(|c| c.into()),
        });
        self
    }

    pub fn build(self) -> StartEvent {
        StartEvent {
            id: self.id,
            name: self.name,
            event_definition: self.event_definition,
        }
    }
}

/// End event builder
pub struct EndEventBuilder {
    id: ElementId,
    name: Option<String>,
    event_definition: Option<EventDefinition>,
}

impl EndEventBuilder {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            name: None,
            event_definition: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn message(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Message {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn signal(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Signal {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn error(mut self, error_code: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Error {
            error_code: error_code.map(|c| c.into()),
        });
        self
    }

    pub fn escalation(mut self, escalation_code: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Escalation {
            escalation_code: escalation_code.map(|c| c.into()),
        });
        self
    }

    pub fn build(self) -> EndEvent {
        EndEvent {
            id: self.id,
            name: self.name,
            event_definition: self.event_definition,
        }
    }
}

/// Intermediate event builder
pub struct IntermediateEventBuilder {
    id: ElementId,
    name: Option<String>,
    event_definition: Option<EventDefinition>,
}

impl IntermediateEventBuilder {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            name: None,
            event_definition: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn message(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Message {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn timer(mut self, expression: impl Into<String>) -> Self {
        self.event_definition = Some(EventDefinition::Timer {
            expression: expression.into(),
        });
        self
    }

    pub fn signal(mut self, name: Option<impl Into<String>>) -> Self {
        self.event_definition = Some(EventDefinition::Signal {
            name: name.map(|n| n.into()),
        });
        self
    }

    pub fn build(self) -> IntermediateEvent {
        IntermediateEvent {
            id: self.id,
            name: self.name,
            event_definition: self.event_definition,
        }
    }
}

