//! Task builders (user, service, script, manual).

use crate::core::elements::{UserTask, ServiceTask, ScriptTask, ManualTask, BpmnElement};
use crate::core::types::ActivityId;
use crate::dsl::process::ProcessBuilder;

/// User task builder
pub struct UserTaskBuilder<PB> {
    id: ActivityId,
    name: Option<String>,
    assignee: Option<String>,
    candidate_users: Vec<String>,
    candidate_groups: Vec<String>,
    form_key: Option<String>,
    due_date: Option<String>,
    parent: PB,
}

impl<PB> UserTaskBuilder<PB> {
    pub fn new(id: impl Into<ActivityId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            assignee: None,
            candidate_users: Vec::new(),
            candidate_groups: Vec::new(),
            form_key: None,
            due_date: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn candidate_user(mut self, user: impl Into<String>) -> Self {
        self.candidate_users.push(user.into());
        self
    }

    pub fn candidate_group(mut self, group: impl Into<String>) -> Self {
        self.candidate_groups.push(group.into());
        self
    }

    pub fn form_key(mut self, form_key: impl Into<String>) -> Self {
        self.form_key = Some(form_key.into());
        self
    }

    pub fn due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }

    pub fn build_task(self) -> UserTask {
        UserTask {
            id: self.id,
            name: self.name,
            assignee: self.assignee,
            candidate_users: self.candidate_users,
            candidate_groups: self.candidate_groups,
            form_key: self.form_key,
            due_date: self.due_date,
        }
    }
}

// Implement for ProcessBuilder specifically
impl<S: crate::dsl::process::ProcessState> UserTaskBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let UserTaskBuilder {
            id,
            name,
            assignee,
            candidate_users,
            candidate_groups,
            form_key,
            due_date,
            parent,
        } = self;
        let task = UserTask {
            id,
            name,
            assignee,
            candidate_users,
            candidate_groups,
            form_key,
            due_date,
        };
        parent.add_element(BpmnElement::UserTask(task))
    }
}

/// Service task builder
pub struct ServiceTaskBuilder<PB> {
    id: ActivityId,
    name: Option<String>,
    implementation: Option<String>,
    implementation_type: Option<String>,
    parent: PB,
}

impl<PB> ServiceTaskBuilder<PB> {
    pub fn new(id: impl Into<ActivityId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            implementation: None,
            implementation_type: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn implementation(mut self, implementation: impl Into<String>) -> Self {
        self.implementation = Some(implementation.into());
        self
    }

    pub fn implementation_type(mut self, implementation_type: impl Into<String>) -> Self {
        self.implementation_type = Some(implementation_type.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> ServiceTaskBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let task = ServiceTask {
            id: self.id,
            name: self.name,
            implementation: self.implementation,
            implementation_type: self.implementation_type,
        };
        self.parent.add_element(BpmnElement::ServiceTask(task))
    }
}

/// Script task builder
pub struct ScriptTaskBuilder<PB> {
    id: ActivityId,
    name: Option<String>,
    script: Option<String>,
    script_format: Option<String>,
    parent: PB,
}

impl<PB> ScriptTaskBuilder<PB> {
    pub fn new(id: impl Into<ActivityId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            script: None,
            script_format: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn script(mut self, script: impl Into<String>) -> Self {
        self.script = Some(script.into());
        self
    }

    pub fn script_format(mut self, script_format: impl Into<String>) -> Self {
        self.script_format = Some(script_format.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> ScriptTaskBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let task = ScriptTask {
            id: self.id,
            name: self.name,
            script: self.script.unwrap_or_else(|| String::new()),
            script_format: self.script_format,
        };
        self.parent.add_element(BpmnElement::ScriptTask(task))
    }
}

/// Manual task builder
pub struct ManualTaskBuilder<PB> {
    id: ActivityId,
    name: Option<String>,
    parent: PB,
}

impl<PB> ManualTaskBuilder<PB> {
    pub fn new(id: impl Into<ActivityId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> ManualTaskBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let task = ManualTask {
            id: self.id,
            name: self.name,
        };
        self.parent.add_element(BpmnElement::ManualTask(task))
    }
}

