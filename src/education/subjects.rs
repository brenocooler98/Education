use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubjectCategory {
    Mathematics,
    Portuguese,
    Science,
    History,
    Geography,
    Programming,
    Other,
}

impl SubjectCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mathematics => "Matemática",
            Self::Portuguese => "Português",
            Self::Science => "Ciências",
            Self::History => "História",
            Self::Geography => "Geografia",
            Self::Programming => "Programação",
            Self::Other => "Outros",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Mathematics => "📐",
            Self::Portuguese => "📖",
            Self::Science => "🔬",
            Self::History => "🏛",
            Self::Geography => "🌎",
            Self::Programming => "💻",
            Self::Other => "📚",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Mathematics,
            Self::Portuguese,
            Self::Science,
            Self::History,
            Self::Geography,
            Self::Programming,
            Self::Other,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SubjectCategory,
    pub lesson_count: u32,
    pub enabled: bool,
}

impl Subject {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        category: SubjectCategory,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            category,
            lesson_count: 0,
            enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_lesson_count(&mut self, count: u32) {
        self.lesson_count = count;
    }

    pub fn display_name(&self) -> String {
        format!("{} {}", self.category.icon(), self.name)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubjectManager {
    subjects: Vec<Subject>,
}

impl SubjectManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut manager = Self::new();

        let defaults = [
            (
                "mathematics",
                "Matemática",
                "Álgebra, geometria, funções e cálculos.",
                SubjectCategory::Mathematics,
            ),
            (
                "portuguese",
                "Português",
                "Gramática, interpretação e produção textual.",
                SubjectCategory::Portuguese,
            ),
            (
                "science",
                "Ciências",
                "Biologia, física, química e natureza.",
                SubjectCategory::Science,
            ),
            (
                "history",
                "História",
                "História do Brasil e história mundial.",
                SubjectCategory::History,
            ),
            (
                "geography",
                "Geografia",
                "Geografia física, humana e ambiental.",
                SubjectCategory::Geography,
            ),
            (
                "programming",
                "Programação",
                "Lógica, algoritmos e linguagens de programação.",
                SubjectCategory::Programming,
            ),
        ];

        for (id, name, description, category) in defaults {
            manager.add(Subject::new(
                id,
                name,
                description,
                category,
            ));
        }

        manager
    }

    pub fn add(&mut self, subject: Subject) -> bool {
        if self.contains(&subject.id) {
            return false;
        }

        self.subjects.push(subject);
        true
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let original_len = self.subjects.len();

        self.subjects.retain(|subject| subject.id != id);

        self.subjects.len() != original_len
    }

    pub fn get(&self, id: &str) -> Option<&Subject> {
        self.subjects.iter().find(|subject| subject.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Subject> {
        self.subjects
            .iter_mut()
            .find(|subject| subject.id == id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    pub fn all(&self) -> &[Subject] {
        &self.subjects
    }

    pub fn enabled(&self) -> Vec<&Subject> {
        self.subjects
            .iter()
            .filter(|subject| subject.enabled)
            .collect()
    }

    pub fn by_category(
        &self,
        category: SubjectCategory,
    ) -> Vec<&Subject> {
        self.subjects
            .iter()
            .filter(|subject| subject.category == category)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Subject> {
        let query = query.trim().to_lowercase();

        if query.is_empty() {
            return self.subjects.iter().collect();
        }

        self.subjects
            .iter()
            .filter(|subject| {
                subject.name.to_lowercase().contains(&query)
                    || subject.description.to_lowercase().contains(&query)
                    || subject.id.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.subjects.len()
    }

    pub fn enabled_count(&self) -> usize {
        self.subjects
            .iter()
            .filter(|subject| subject.enabled)
            .count()
    }

    pub fn total_lessons(&self) -> u32 {
        self.subjects
            .iter()
            .map(|subject| subject.lesson_count)
            .sum()
    }

    pub fn clear(&mut self) {
        self.subjects.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_subjects() {
        let manager = SubjectManager::with_defaults();

        assert_eq!(manager.count(), 6);
        assert!(manager.contains("mathematics"));
        assert!(manager.contains("programming"));
    }

    #[test]
    fn test_add_and_remove() {
        let mut manager = SubjectManager::new();

        let subject = Subject::new(
            "rust",
            "Rust",
            "Aprenda a linguagem Rust.",
            SubjectCategory::Programming,
        );

        assert!(manager.add(subject));
        assert!(!manager.contains("unknown"));
        assert!(manager.contains("rust"));

        assert!(manager.remove("rust"));
        assert!(!manager.contains("rust"));
    }

    #[test]
    fn test_duplicate_subject() {
        let mut manager = SubjectManager::new();

        let subject = Subject::new(
            "math",
            "Matemática",
            "Matemática básica.",
            SubjectCategory::Mathematics,
        );

        assert!(manager.add(subject.clone()));
        assert!(!manager.add(subject));

        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_search() {
        let manager = SubjectManager::with_defaults();

        let results = manager.search("Matemática");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "mathematics");
    }

    #[test]
    fn test_categories() {
        let manager = SubjectManager::with_defaults();

        let programming = manager.by_category(
            SubjectCategory::Programming
        );

        assert_eq!(programming.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let manager = SubjectManager::with_defaults();

        let json = serde_json::to_string(&manager).unwrap();

        let restored: SubjectManager =
            serde_json::from_str(&json).unwrap();

        assert_eq!(restored.count(), manager.count());
    }
}
