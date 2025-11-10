//! BPMN process definition in IR format.

use crate::core::elements::BpmnElement;
use crate::core::types::{ProcessId, ValidationState};
use serde::{Serialize, Deserialize};

/// BPMN Internal Representation with phantom types for validation state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnIR<S: ValidationState> {
    /// Process identifier
    pub process_id: ProcessId,
    /// Process name
    pub name: Option<String>,
    /// Process elements (events, tasks, gateways, flows)
    pub elements: Vec<BpmnElement>,
    /// Process variables/context
    pub variables: Vec<ProcessVariable>,
    /// Validation state marker (phantom type)
    #[serde(skip)]
    _state: std::marker::PhantomData<S>,
}

impl<S: ValidationState> BpmnIR<S> {
    /// Create a new BPMN IR instance
    pub fn new(process_id: ProcessId) -> Self {
        Self {
            process_id,
            name: None,
            elements: Vec::new(),
            variables: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }

    /// Set the process name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add an element to the process
    pub fn add_element(mut self, element: BpmnElement) -> Self {
        self.elements.push(element);
        self
    }

    /// Add a process variable
    pub fn add_variable(mut self, variable: ProcessVariable) -> Self {
        self.variables.push(variable);
        self
    }

    /// Get all elements
    pub fn elements(&self) -> &[BpmnElement] {
        &self.elements
    }

    /// Get process ID
    pub fn process_id(&self) -> &ProcessId {
        &self.process_id
    }

    /// Convert to a different validation state (internal helper)
    pub(crate) fn into_state<T: ValidationState>(self) -> BpmnIR<T> {
        BpmnIR {
            process_id: self.process_id,
            name: self.name,
            elements: self.elements,
            variables: self.variables,
            _state: std::marker::PhantomData,
        }
    }
}

/// Process variable definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessVariable {
    pub name: String,
    pub data_type: Option<String>,
    pub default_value: Option<String>,
}

impl ProcessVariable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data_type: None,
            default_value: None,
        }
    }

    pub fn with_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }

    pub fn with_default(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

