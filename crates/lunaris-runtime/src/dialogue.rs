//! Dialogue System
//!
//! Visual dialogue trees and branching conversations.

use std::collections::HashMap;

/// Dialogue tree
pub struct DialogueTree {
    pub id: u64,
    pub name: String,
    pub nodes: HashMap<u64, DialogueNode>,
    pub start_node: u64,
    pub variables: HashMap<String, DialogueValue>,
    pub speakers: Vec<Speaker>,
}

/// Dialogue node
pub struct DialogueNode {
    pub id: u64,
    pub node_type: NodeType,
    pub position: (f32, f32),
}

/// Node type
pub enum NodeType {
    Dialogue { speaker: u64, text: String, voice_clip: Option<String>, next: u64 },
    Choice { prompt: String, options: Vec<DialogueChoice> },
    Condition { variable: String, operator: ConditionOp, value: DialogueValue, if_true: u64, if_false: u64 },
    SetVariable { variable: String, value: DialogueValue, next: u64 },
    Event { event_name: String, parameters: HashMap<String, String>, next: u64 },
    Random { options: Vec<(f32, u64)> },
    End,
}

/// Choice option
pub struct DialogueChoice {
    pub text: String,
    pub condition: Option<String>,
    pub next: u64,
}

/// Condition operator
pub enum ConditionOp { Equals, NotEquals, Greater, Less, GreaterEquals, LessEquals }

/// Dialogue value
#[derive(Clone)]
pub enum DialogueValue { Bool(bool), Int(i32), Float(f32), String(String) }

/// Speaker
pub struct Speaker {
    pub id: u64,
    pub name: String,
    pub portrait: Option<String>,
    pub color: [f32; 3],
}

/// Dialogue runtime
pub struct DialogueRuntime {
    pub tree: Option<DialogueTree>,
    pub current_node: u64,
    pub state: DialogueState,
    pub history: Vec<DialogueHistoryEntry>,
}

/// Dialogue state
pub enum DialogueState { Inactive, Speaking { text: String, speaker: Speaker, char_index: usize }, WaitingChoice { options: Vec<DialogueChoice> }, Finished }

/// History entry
pub struct DialogueHistoryEntry {
    pub speaker: String,
    pub text: String,
}

impl DialogueTree {
    pub fn new(name: &str) -> Self {
        Self { id: 1, name: name.into(), nodes: HashMap::new(), start_node: 0, variables: HashMap::new(), speakers: Vec::new() }
    }

    pub fn add_dialogue(&mut self, id: u64, speaker: u64, text: &str, next: u64) {
        self.nodes.insert(id, DialogueNode { id, node_type: NodeType::Dialogue { speaker, text: text.into(), voice_clip: None, next }, position: (0.0, 0.0) });
    }

    pub fn add_choice(&mut self, id: u64, prompt: &str, options: Vec<DialogueChoice>) {
        self.nodes.insert(id, DialogueNode { id, node_type: NodeType::Choice { prompt: prompt.into(), options }, position: (0.0, 0.0) });
    }
}

impl DialogueRuntime {
    pub fn new() -> Self { Self { tree: None, current_node: 0, state: DialogueState::Inactive, history: Vec::new() } }

    pub fn start(&mut self, tree: DialogueTree) {
        self.current_node = tree.start_node;
        self.tree = Some(tree);
        self.advance();
    }

    pub fn advance(&mut self) {
        let Some(tree) = &self.tree else { return };
        let Some(node) = tree.nodes.get(&self.current_node) else { self.state = DialogueState::Finished; return };

        match &node.node_type {
            NodeType::Dialogue { speaker, text, next, .. } => {
                let speaker_data = tree.speakers.iter().find(|s| s.id == *speaker).cloned().unwrap_or(Speaker { id: 0, name: "Unknown".into(), portrait: None, color: [1.0; 3] });
                self.history.push(DialogueHistoryEntry { speaker: speaker_data.name.clone(), text: text.clone() });
                self.state = DialogueState::Speaking { text: text.clone(), speaker: speaker_data, char_index: 0 };
                self.current_node = *next;
            }
            NodeType::Choice { options, .. } => {
                self.state = DialogueState::WaitingChoice { options: options.clone() };
            }
            NodeType::End => {
                self.state = DialogueState::Finished;
            }
            _ => {}
        }
    }

    pub fn choose(&mut self, index: usize) {
        if let DialogueState::WaitingChoice { options } = &self.state {
            if let Some(choice) = options.get(index) {
                self.current_node = choice.next;
                self.advance();
            }
        }
    }

    pub fn skip(&mut self) {
        if matches!(self.state, DialogueState::Speaking { .. }) {
            self.advance();
        }
    }
}

impl Clone for Speaker {
    fn clone(&self) -> Self { Self { id: self.id, name: self.name.clone(), portrait: self.portrait.clone(), color: self.color } }
}

impl Clone for DialogueChoice {
    fn clone(&self) -> Self { Self { text: self.text.clone(), condition: self.condition.clone(), next: self.next } }
}
