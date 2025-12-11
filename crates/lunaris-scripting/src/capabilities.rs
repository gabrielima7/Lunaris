//! Capability system for controlling script access
//!
//! Scripts are granted capabilities based on their trust level.
//! Each capability represents access to a specific engine API.

use std::collections::HashSet;

/// Available capabilities that can be granted to scripts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Basic print/logging functions
    Logging,
    /// Read game entities
    EntityRead,
    /// Modify game entities
    EntityWrite,
    /// Perform physics raycasts
    PhysicsRaycast,
    /// Trigger audio playback
    AudioPlay,
    /// Access input state
    Input,
    /// Read game time
    Time,
    /// Access math utilities
    Math,
    /// Read game configuration
    ConfigRead,
    /// Modify game configuration (trusted only)
    ConfigWrite,
    /// Access debug functions (trusted only)
    Debug,
    /// File system access within game directory (trusted only)
    FileSystem,
}

/// Trust level for scripts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Community mods - most restricted
    Untrusted,
    /// Verified/signed mods - some additional access
    Verified,
    /// Developer scripts - full access
    Trusted,
}

impl TrustLevel {
    /// Get the default capabilities for this trust level
    #[must_use]
    pub fn default_capabilities(self) -> HashSet<Capability> {
        let mut caps = HashSet::new();

        // Base capabilities for all scripts
        caps.insert(Capability::Logging);
        caps.insert(Capability::Math);
        caps.insert(Capability::Time);
        caps.insert(Capability::Input);

        if self >= TrustLevel::Untrusted {
            caps.insert(Capability::EntityRead);
            caps.insert(Capability::PhysicsRaycast);
            caps.insert(Capability::AudioPlay);
        }

        if self >= TrustLevel::Verified {
            caps.insert(Capability::EntityWrite);
            caps.insert(Capability::ConfigRead);
        }

        if self >= TrustLevel::Trusted {
            caps.insert(Capability::ConfigWrite);
            caps.insert(Capability::Debug);
            caps.insert(Capability::FileSystem);
        }

        caps
    }
}

/// Manages capability grants for a script context
#[derive(Debug, Clone)]
pub struct CapabilitySet {
    capabilities: HashSet<Capability>,
    trust_level: TrustLevel,
}

impl CapabilitySet {
    /// Create a new capability set from a trust level
    #[must_use]
    pub fn new(trust_level: TrustLevel) -> Self {
        Self {
            capabilities: trust_level.default_capabilities(),
            trust_level,
        }
    }

    /// Check if a capability is granted
    #[must_use]
    pub fn has(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Grant an additional capability
    pub fn grant(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }

    /// Revoke a capability
    pub fn revoke(&mut self, capability: Capability) {
        self.capabilities.remove(&capability);
    }

    /// Get the trust level
    #[must_use]
    pub const fn trust_level(&self) -> TrustLevel {
        self.trust_level
    }

    /// Get all granted capabilities
    #[must_use]
    pub fn capabilities(&self) -> &HashSet<Capability> {
        &self.capabilities
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new(TrustLevel::Untrusted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untrusted_has_base_capabilities() {
        let caps = CapabilitySet::new(TrustLevel::Untrusted);
        assert!(caps.has(Capability::Logging));
        assert!(caps.has(Capability::Math));
        assert!(!caps.has(Capability::Debug));
        assert!(!caps.has(Capability::FileSystem));
    }

    #[test]
    fn trusted_has_all_capabilities() {
        let caps = CapabilitySet::new(TrustLevel::Trusted);
        assert!(caps.has(Capability::Logging));
        assert!(caps.has(Capability::Debug));
        assert!(caps.has(Capability::FileSystem));
    }

    #[test]
    fn can_grant_and_revoke() {
        let mut caps = CapabilitySet::new(TrustLevel::Untrusted);
        assert!(!caps.has(Capability::Debug));

        caps.grant(Capability::Debug);
        assert!(caps.has(Capability::Debug));

        caps.revoke(Capability::Debug);
        assert!(!caps.has(Capability::Debug));
    }
}
