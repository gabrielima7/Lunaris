//! AI Copilot for Visual Scripting
//!
//! LLM-powered assistant for generating Blueprints and code.

use std::collections::HashMap;

/// AI Backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIBackend {
    /// Local Llama model
    Llama,
    /// Local Mistral model
    Mistral,
    /// OpenAI API
    OpenAI,
    /// Anthropic API
    Anthropic,
    /// Local Ollama
    Ollama,
    /// Custom endpoint
    Custom,
}

/// AI Copilot configuration
#[derive(Debug, Clone)]
pub struct CopilotConfig {
    /// Backend to use
    pub backend: AIBackend,
    /// Model name
    pub model: String,
    /// API endpoint (for custom/local)
    pub endpoint: String,
    /// API key (for cloud services)
    pub api_key: Option<String>,
    /// Max tokens
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
    /// Context window size
    pub context_size: u32,
    /// Enable code suggestions
    pub code_suggestions: bool,
    /// Enable blueprint generation
    pub blueprint_generation: bool,
    /// Enable voice commands
    pub voice_commands: bool,
}

impl Default for CopilotConfig {
    fn default() -> Self {
        Self {
            backend: AIBackend::Ollama,
            model: "llama3.2".to_string(),
            endpoint: "http://localhost:11434".to_string(),
            api_key: None,
            max_tokens: 2048,
            temperature: 0.7,
            context_size: 8192,
            code_suggestions: true,
            blueprint_generation: true,
            voice_commands: false,
        }
    }
}

/// Blueprint suggestion
#[derive(Debug, Clone)]
pub struct BlueprintSuggestion {
    /// Suggestion ID
    pub id: u64,
    /// Node type
    pub node_type: String,
    /// Display name
    pub display_name: String,
    /// Description
    pub description: String,
    /// Confidence (0-1)
    pub confidence: f32,
    /// Input connections
    pub inputs: Vec<(String, String)>,
    /// Output connections
    pub outputs: Vec<(String, String)>,
    /// Code preview
    pub code_preview: Option<String>,
}

/// Code suggestion
#[derive(Debug, Clone)]
pub struct CodeSuggestion {
    /// Suggestion text
    pub text: String,
    /// Insert position
    pub position: usize,
    /// Confidence
    pub confidence: f32,
    /// Language
    pub language: String,
}

/// Voice command result
#[derive(Debug, Clone)]
pub struct VoiceCommand {
    /// Recognized text
    pub text: String,
    /// Confidence
    pub confidence: f32,
    /// Parsed action
    pub action: CopilotAction,
}

/// Copilot action
#[derive(Debug, Clone)]
pub enum CopilotAction {
    /// Generate blueprint nodes
    GenerateBlueprint(String),
    /// Generate code
    GenerateCode { language: String, description: String },
    /// Explain code
    ExplainCode(String),
    /// Fix error
    FixError { code: String, error: String },
    /// Refactor
    Refactor { code: String, instruction: String },
    /// Search documentation
    SearchDocs(String),
    /// Create asset
    CreateAsset { asset_type: String, name: String },
    /// Unknown
    Unknown(String),
}

/// Conversation message
#[derive(Debug, Clone)]
pub struct Message {
    /// Role
    pub role: MessageRole,
    /// Content
    pub content: String,
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// AI Copilot
pub struct AICopilot {
    /// Configuration
    pub config: CopilotConfig,
    /// Conversation history
    history: Vec<Message>,
    /// System prompt
    system_prompt: String,
    /// Context (current file, node, etc.)
    context: HashMap<String, String>,
    /// Suggestion cache
    cache: Vec<BlueprintSuggestion>,
    /// Is connected
    connected: bool,
    /// Next suggestion ID
    next_id: u64,
}

impl Default for AICopilot {
    fn default() -> Self {
        Self::new(CopilotConfig::default())
    }
}

impl AICopilot {
    /// Create new copilot
    #[must_use]
    pub fn new(config: CopilotConfig) -> Self {
        let system_prompt = r#"You are an AI assistant for the Lunaris Game Engine.
You help developers create games using visual scripting (Blueprints) and Rust/Lua code.

Available APIs:
- lunaris.entity: create, get_position, set_position, move, get_rotation, set_rotation
- lunaris.input: is_key_down, is_key_pressed, get_mouse_position, get_axis
- lunaris.audio: play, stop, set_volume
- lunaris.physics: raycast, check_collision
- lunaris.scene: load, get_current

When generating Blueprints, output JSON with this structure:
{
  "nodes": [
    {"type": "Event", "name": "BeginPlay", "id": 1},
    {"type": "Function", "name": "PrintString", "id": 2, "inputs": {"string": "Hello"}}
  ],
  "connections": [
    {"from": 1, "from_pin": "exec", "to": 2, "to_pin": "exec"}
  ]
}

Be concise and practical."#.to_string();

        Self {
            config,
            history: Vec::new(),
            system_prompt,
            context: HashMap::new(),
            cache: Vec::new(),
            connected: false,
            next_id: 1,
        }
    }

    /// Connect to backend
    pub fn connect(&mut self) -> Result<(), CopilotError> {
        // Would actually connect to the LLM backend
        self.connected = true;
        tracing::info!("AI Copilot connected to {:?}", self.config.backend);
        Ok(())
    }

    /// Add context
    pub fn set_context(&mut self, key: &str, value: &str) {
        self.context.insert(key.to_string(), value.to_string());
    }

    /// Clear context
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// Generate blueprint from natural language
    pub fn generate_blueprint(&mut self, prompt: &str) -> Result<Vec<BlueprintSuggestion>, CopilotError> {
        if !self.connected {
            return Err(CopilotError::NotConnected);
        }

        // Add to history
        self.history.push(Message {
            role: MessageRole::User,
            content: format!("Generate a Blueprint for: {}", prompt),
        });

        // Would call LLM API here
        // For now, return example suggestions based on keywords

        let mut suggestions = Vec::new();

        if prompt.to_lowercase().contains("move") || prompt.to_lowercase().contains("movement") {
            suggestions.push(BlueprintSuggestion {
                id: self.next_id(),
                node_type: "Event".to_string(),
                display_name: "Event Tick".to_string(),
                description: "Called every frame".to_string(),
                confidence: 0.95,
                inputs: vec![],
                outputs: vec![
                    ("exec".to_string(), "Execution".to_string()),
                    ("delta".to_string(), "Float".to_string()),
                ],
                code_preview: None,
            });

            suggestions.push(BlueprintSuggestion {
                id: self.next_id(),
                node_type: "Function".to_string(),
                display_name: "Add Movement Input".to_string(),
                description: "Move character in direction".to_string(),
                confidence: 0.9,
                inputs: vec![
                    ("exec".to_string(), "Execution".to_string()),
                    ("direction".to_string(), "Vector3".to_string()),
                    ("scale".to_string(), "Float".to_string()),
                ],
                outputs: vec![("exec".to_string(), "Execution".to_string())],
                code_preview: Some("lunaris.entity.move(self, dx, dy)".to_string()),
            });
        }

        if prompt.to_lowercase().contains("jump") {
            suggestions.push(BlueprintSuggestion {
                id: self.next_id(),
                node_type: "Function".to_string(),
                display_name: "Jump".to_string(),
                description: "Make character jump".to_string(),
                confidence: 0.88,
                inputs: vec![
                    ("exec".to_string(), "Execution".to_string()),
                    ("force".to_string(), "Float".to_string()),
                ],
                outputs: vec![("exec".to_string(), "Execution".to_string())],
                code_preview: Some("lunaris.physics.apply_impulse(self, 0, jump_force)".to_string()),
            });
        }

        if prompt.to_lowercase().contains("shoot") || prompt.to_lowercase().contains("fire") {
            suggestions.push(BlueprintSuggestion {
                id: self.next_id(),
                node_type: "Function".to_string(),
                display_name: "Spawn Projectile".to_string(),
                description: "Spawn a projectile actor".to_string(),
                confidence: 0.85,
                inputs: vec![
                    ("exec".to_string(), "Execution".to_string()),
                    ("class".to_string(), "Class".to_string()),
                    ("location".to_string(), "Vector3".to_string()),
                ],
                outputs: vec![
                    ("exec".to_string(), "Execution".to_string()),
                    ("actor".to_string(), "Actor".to_string()),
                ],
                code_preview: Some("local bullet = lunaris.entity.create('Bullet')".to_string()),
            });
        }

        // Add to history
        self.history.push(Message {
            role: MessageRole::Assistant,
            content: format!("Generated {} blueprint suggestions", suggestions.len()),
        });

        self.cache = suggestions.clone();
        Ok(suggestions)
    }

    /// Generate code from description
    pub fn generate_code(&mut self, description: &str, language: &str) -> Result<String, CopilotError> {
        if !self.connected {
            return Err(CopilotError::NotConnected);
        }

        self.history.push(Message {
            role: MessageRole::User,
            content: format!("Generate {} code for: {}", language, description),
        });

        // Would call LLM API
        // For now, return template based on language

        let code = match language {
            "lua" => {
                format!(
                    r#"-- {}
function on_update(dt)
    -- TODO: Implement
end

function on_begin_play()
    print("Script initialized")
end
"#,
                    description
                )
            }
            "rust" => {
                format!(
                    r#"//! {}

use lunaris_runtime::Application;

pub struct MyComponent {{
    // TODO: Add fields
}}

impl MyComponent {{
    pub fn new() -> Self {{
        Self {{}}
    }}

    pub fn update(&mut self, dt: f32) {{
        // TODO: Implement
    }}
}}
"#,
                    description
                )
            }
            _ => format!("// {}\n// Language not supported", description),
        };

        self.history.push(Message {
            role: MessageRole::Assistant,
            content: code.clone(),
        });

        Ok(code)
    }

    /// Explain code
    pub fn explain_code(&mut self, code: &str) -> Result<String, CopilotError> {
        if !self.connected {
            return Err(CopilotError::NotConnected);
        }

        // Would call LLM
        // Return template explanation
        let explanation = format!(
            "This code appears to:\n1. Define a component or system\n2. Handle game logic\n3. Interact with the Lunaris API\n\nCode length: {} characters",
            code.len()
        );

        Ok(explanation)
    }

    /// Fix error in code
    pub fn fix_error(&mut self, code: &str, error: &str) -> Result<String, CopilotError> {
        if !self.connected {
            return Err(CopilotError::NotConnected);
        }

        // Would call LLM
        // Return with common fixes applied
        let fixed = code.replace("funtion", "function")
            .replace("retrun", "return")
            .replace("pritn", "print");

        Ok(fixed)
    }

    /// Parse voice command
    pub fn parse_voice_command(&self, text: &str) -> VoiceCommand {
        let text_lower = text.to_lowercase();
        
        let action = if text_lower.starts_with("create") {
            if text_lower.contains("blueprint") {
                CopilotAction::GenerateBlueprint(text.to_string())
            } else if text_lower.contains("script") || text_lower.contains("code") {
                CopilotAction::GenerateCode {
                    language: "lua".to_string(),
                    description: text.to_string(),
                }
            } else {
                CopilotAction::CreateAsset {
                    asset_type: "unknown".to_string(),
                    name: text.to_string(),
                }
            }
        } else if text_lower.starts_with("explain") {
            CopilotAction::ExplainCode(text.to_string())
        } else if text_lower.starts_with("fix") {
            CopilotAction::FixError {
                code: String::new(),
                error: text.to_string(),
            }
        } else if text_lower.starts_with("search") || text_lower.starts_with("find") {
            CopilotAction::SearchDocs(text.to_string())
        } else {
            CopilotAction::Unknown(text.to_string())
        };

        VoiceCommand {
            text: text.to_string(),
            confidence: 0.8,
            action,
        }
    }

    /// Get auto-complete suggestions for current context
    pub fn get_suggestions(&self, partial: &str, max: usize) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();

        // Lunaris API completions
        let api_completions = [
            ("lunaris.entity.", "lunaris.entity.create"),
            ("lunaris.entity.", "lunaris.entity.get_position"),
            ("lunaris.entity.", "lunaris.entity.set_position"),
            ("lunaris.entity.", "lunaris.entity.move"),
            ("lunaris.input.", "lunaris.input.is_key_down"),
            ("lunaris.input.", "lunaris.input.is_key_pressed"),
            ("lunaris.input.", "lunaris.input.get_mouse_position"),
            ("lunaris.audio.", "lunaris.audio.play"),
            ("lunaris.audio.", "lunaris.audio.stop"),
            ("lunaris.physics.", "lunaris.physics.raycast"),
            ("lunaris.scene.", "lunaris.scene.load"),
        ];

        for (prefix, completion) in api_completions {
            if partial.ends_with(prefix) || completion.starts_with(partial) {
                suggestions.push(CodeSuggestion {
                    text: completion.to_string(),
                    position: 0,
                    confidence: 0.9,
                    language: "lua".to_string(),
                });
            }
        }

        suggestions.truncate(max);
        suggestions
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

/// Copilot error
#[derive(Debug, Clone)]
pub enum CopilotError {
    /// Not connected to backend
    NotConnected,
    /// API error
    ApiError(String),
    /// Rate limited
    RateLimited,
    /// Invalid response
    InvalidResponse,
    /// Timeout
    Timeout,
}

impl std::fmt::Display for CopilotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConnected => write!(f, "Not connected to AI backend"),
            Self::ApiError(e) => write!(f, "API error: {}", e),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::InvalidResponse => write!(f, "Invalid response from AI"),
            Self::Timeout => write!(f, "Request timed out"),
        }
    }
}

impl std::error::Error for CopilotError {}
