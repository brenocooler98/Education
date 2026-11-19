pub mod subjects;
pub mod lessons;
pub mod exercises;
pub mod progress;

use serde::{Deserialize, Serialize};

pub use subjects::*;
pub use lessons::*;
pub use exercises::*;
pub use progress::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Expert => "Expert",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Beginner => 1.0,
            Self::Intermediate => 1.5,
            Self::Advanced => 2.0,
            Self::Expert => 3.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EducationLevel {
    Elementary,
    MiddleSchool,
    HighSchool,
    University,
}

impl EducationLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Elementary => "Elementary School",
            Self::MiddleSchool => "Middle School",
            Self::HighSchool => "High School",
            Self::University => "University",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationConfig {
    pub language: String,
    pub level: EducationLevel,
    pub default_difficulty: Difficulty,
    pub enable_exercises: bool,
    pub enable_progress: bool,
    pub enable_achievements: bool,
    pub enable_offline_learning: bool,
}

impl Default for EducationConfig {
    fn default() -> Self {
        Self {
            language: "pt-BR".to_string(),
            level: EducationLevel::HighSchool,
            default_difficulty: Difficulty::Beginner,
            enable_exercises: true,
            enable_progress: true,
            enable_achievements: true,
            enable_offline_learning: true,
        }
    }
}

pub struct EducationSystem {
    config: EducationConfig,
    initialized: bool,
}

impl Default for EducationSystem {
    fn default() -> Self {
        Self::new(EducationConfig::default())
    }
}

impl EducationSystem {
    pub fn new(config: EducationConfig) -> Self {
        Self {
            config,
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        log::info!("Initializing Education System");

        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn config(&self) -> &EducationConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut EducationConfig {
        &mut self.config
    }

    pub fn set_language(&mut self, language: impl Into<String>) {
        self.config.language = language.into();
    }

    pub fn set_level(&mut self, level: EducationLevel) {
        self.config.level = level;
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.config.default_difficulty = difficulty;
    }

    pub fn shutdown(&mut self) {
        if !self.initialized {
            return;
        }

        log::info!("Shutting down Education System");

        self.initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EducationConfig::default();

        assert_eq!(config.language, "pt-BR");
        assert!(config.enable_exercises);
        assert!(config.enable_offline_learning);
    }

    #[test]
    fn test_system_initialization() {
        let mut system = EducationSystem::default();

        assert!(!system.is_initialized());

        system.initialize();

        assert!(system.is_initialized());

        system.shutdown();

        assert!(!system.is_initialized());
    }

    #[test]
    fn test_difficulty_multiplier() {
        assert_eq!(Difficulty::Beginner.multiplier(), 1.0);
        assert_eq!(Difficulty::Expert.multiplier(), 3.0);
    }
}
