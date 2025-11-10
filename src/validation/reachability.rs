//! Reachability analysis for BPMN processes.

use crate::core::{BpmnIR, Unvalidated};
use crate::core::elements::BpmnElement;
use std::collections::{HashMap, HashSet, VecDeque};

/// Reachability analysis result
#[derive(Debug, Clone)]
pub struct ReachabilityResult {
    pub all_reachable: bool,
    pub unreachable_elements: Vec<String>,
    pub reachable_from_start: HashSet<String>,
}

/// Reachability analyzer
pub struct ReachabilityAnalyzer;

impl ReachabilityAnalyzer {
    /// Analyze reachability of elements from start events
    pub fn analyze(ir: &BpmnIR<Unvalidated>) -> ReachabilityResult {
        // Build adjacency list for sequence flows
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_elements: HashSet<String> = HashSet::new();
        let mut start_elements: Vec<String> = Vec::new();

        // Collect all element IDs
        for element in ir.elements() {
            let id = match element {
                BpmnElement::StartEvent(se) => {
                    start_elements.push(se.id.as_str().to_string());
                    se.id.as_str().to_string()
                }
                BpmnElement::EndEvent(ee) => ee.id.as_str().to_string(),
                BpmnElement::UserTask(ut) => ut.id.as_str().to_string(),
                BpmnElement::ServiceTask(st) => st.id.as_str().to_string(),
                BpmnElement::ScriptTask(st) => st.id.as_str().to_string(),
                BpmnElement::ManualTask(mt) => mt.id.as_str().to_string(),
                BpmnElement::ExclusiveGateway(eg) => eg.id.as_str().to_string(),
                BpmnElement::ParallelGateway(pg) => pg.id.as_str().to_string(),
                BpmnElement::InclusiveGateway(ig) => ig.id.as_str().to_string(),
                BpmnElement::SequenceFlow(_) => continue, // Skip flows
                _ => continue,
            };
            all_elements.insert(id.clone());
        }

        // Build graph from sequence flows
        for element in ir.elements() {
            if let BpmnElement::SequenceFlow(flow) = element {
                graph
                    .entry(flow.source_ref.as_str().to_string())
                    .or_insert_with(Vec::new)
                    .push(flow.target_ref.as_str().to_string());
            }
        }

        // BFS from all start events
        let mut reachable: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();

        for start in &start_elements {
            reachable.insert(start.clone());
            queue.push_back(start.clone());
        }

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if !reachable.contains(neighbor) {
                        reachable.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        // Find unreachable elements
        let unreachable: Vec<String> = all_elements
            .iter()
            .filter(|id| !reachable.contains(*id))
            .cloned()
            .collect();

        ReachabilityResult {
            all_reachable: unreachable.is_empty(),
            unreachable_elements: unreachable,
            reachable_from_start: reachable,
        }
    }
}

