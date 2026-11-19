use serde::{Deserialize, Serialize};

use super::Difficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonTopic {
    pub title: String,
    pub content: String,
}

impl LessonTopic {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub subject_id: String,
    pub title: String,
    pub description: String,
    pub topics: Vec<LessonTopic>,
    pub difficulty: Difficulty,
    pub duration_minutes: u32,
    pub xp_reward: u32,
    pub status: LessonStatus,
    pub enabled: bool,
}

impl Lesson {
    pub fn new(
        id: impl Into<String>,
        subject_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            id: id.into(),
            subject_id: subject_id.into(),
            title: title.into(),
            description: description.into(),
            topics: Vec::new(),
            difficulty,
            duration_minutes: 10,
            xp_reward: 25,
            status: LessonStatus::NotStarted,
            enabled: true,
        }
    }

    pub fn add_topic(&mut self, topic: LessonTopic) {
        self.topics.push(topic);
    }

    pub fn set_duration(&mut self, minutes: u32) {
        self.duration_minutes = minutes;
    }

    pub fn set_xp_reward(&mut self, xp: u32) {
        self.xp_reward = xp;
    }

    pub fn start(&mut self) {
        if self.enabled && self.status == LessonStatus::NotStarted {
            self.status = LessonStatus::InProgress;
        }
    }

    pub fn complete(&mut self) -> bool {
        if !self.enabled || self.status == LessonStatus::Completed {
            return false;
        }

        self.status = LessonStatus::Completed;
        true
    }

    pub fn reset(&mut self) {
        self.status = LessonStatus::NotStarted;
    }

    pub fn is_completed(&self) -> bool {
        self.status == LessonStatus::Completed
    }

    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    pub fn total_characters(&self) -> usize {
        self.topics
            .iter()
            .map(|topic| topic.content.chars().count())
            .sum()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LessonManager {
    lessons: Vec<Lesson>,
}

impl LessonManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut manager = Self::new();

        let mut pythagoras = Lesson::new(
            "math_pythagoras",
            "mathematics",
            "Teorema de Pitágoras",
            "Aprenda a calcular os lados de um triângulo retângulo.",
            Difficulty::Beginner,
        );

        pythagoras.set_duration(15);
        pythagoras.set_xp_reward(50);

        pythagoras.add_topic(LessonTopic::new(
            "Introdução",
            "O Teorema de Pitágoras é utilizado em triângulos retângulos.",
        ));

        pythagoras.add_topic(LessonTopic::new(
            "Fórmula",
            "a² + b² = c², onde c representa a hipotenusa.",
        ));

        pythagoras.add_topic(LessonTopic::new(
            "Exemplo",
            "Se os catetos medem 3 e 4, a hipotenusa mede 5.",
        ));

        manager.add(pythagoras);

        let mut quadratic = Lesson::new(
            "math_quadratic",
            "mathematics",
            "Função Quadrática",
            "Entenda funções do segundo grau.",
            Difficulty::Intermediate,
        );

        quadratic.set_duration(20);
        quadratic.set_xp_reward(75);

        quadratic.add_topic(LessonTopic::new(
            "Definição",
            "Uma função quadrática possui a forma f(x) = ax² + bx + c, com a diferente de zero.",
        ));

        quadratic.add_topic(LessonTopic::new(
            "Discriminante",
            "O discriminante é calculado por Δ = b² - 4ac.",
        ));

        manager.add(quadratic);

        let mut rust = Lesson::new(
            "programming_rust",
            "programming",
            "Introdução ao Rust",
            "Aprenda os fundamentos da linguagem Rust.",
            Difficulty::Beginner,
        );

        rust.set_duration(25);
        rust.set_xp_reward(60);

        rust.add_topic(LessonTopic::new(
            "Hello World",
            "fn main() { println!(\"Hello, World!\"); }",
        ));

        rust.add_topic(LessonTopic::new(
            "Variáveis",
            "Rust utiliza let para declarar variáveis e let mut para variáveis mutáveis.",
        ));

        manager.add(rust);

        manager
    }

    pub fn add(&mut self, lesson: Lesson) -> bool {
        if self.contains(&lesson.id) {
            return false;
        }

        self.lessons.push(lesson);
        true
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.lessons.len();

        self.lessons.retain(|lesson| lesson.id != id);

        self.lessons.len() != before
    }

    pub fn get(&self, id: &str) -> Option<&Lesson> {
        self.lessons.iter().find(|lesson| lesson.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Lesson> {
        self.lessons.iter_mut().find(|lesson| lesson.id == id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    pub fn all(&self) -> &[Lesson] {
        &self.lessons
    }

    pub fn by_subject(&self, subject_id: &str) -> Vec<&Lesson> {
        self.lessons
            .iter()
            .filter(|lesson| lesson.subject_id == subject_id)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Lesson> {
        let query = query.trim().to_lowercase();

        self.lessons
            .iter()
            .filter(|lesson| {
                lesson.title.to_lowercase().contains(&query)
                    || lesson.description.to_lowercase().contains(&query)
                    || lesson.id.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn completed(&self) -> Vec<&Lesson> {
        self.lessons
            .iter()
            .filter(|lesson| lesson.is_completed())
            .collect()
    }

    pub fn available(&self) -> Vec<&Lesson> {
        self.lessons
            .iter()
            .filter(|lesson| lesson.enabled)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.lessons.len()
    }

    pub fn completed_count(&self) -> usize {
        self.completed().len()
    }

    pub fn completion_percentage(&self) -> f32 {
        if self.lessons.is_empty() {
            return 0.0;
        }

        self.completed_count() as f32
            / self.lessons.len() as f32
            * 100.0
    }

    pub fn total_duration(&self) -> u32 {
        self.lessons
            .iter()
            .map(|lesson| lesson.duration_minutes)
            .sum()
    }

    pub fn earned_xp(&self) -> u32 {
        self.lessons
            .iter()
            .filter(|lesson| lesson.is_completed())
            .map(|lesson| lesson.xp_reward)
            .sum()
    }

    pub fn clear(&mut self) {
        self.lessons.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lessons() {
        let manager = LessonManager::with_defaults();

        assert_eq!(manager.count(), 3);
        assert!(manager.contains("math_pythagoras"));
    }

    #[test]
    fn test_lesson_completion() {
        let mut manager = LessonManager::with_defaults();

        let lesson = manager.get_mut("math_pythagoras").unwrap();

        lesson.start();

        assert_eq!(lesson.status, LessonStatus::InProgress);

        assert!(lesson.complete());
        assert!(!lesson.complete());

        assert!(lesson.is_completed());
        assert_eq!(manager.earned_xp(), 50);
    }

    #[test]
    fn test_subject_filter() {
        let manager = LessonManager::with_defaults();

        assert_eq!(manager.by_subject("mathematics").len(), 2);
        assert_eq!(manager.by_subject("programming").len(), 1);
    }

    #[test]
    fn test_serialization() {
        let manager = LessonManager::with_defaults();

        let json = serde_json::to_string(&manager).unwrap();

        let restored: LessonManager =
            serde_json::from_str(&json).unwrap();

        assert_eq!(restored.count(), manager.count());
    }
}
