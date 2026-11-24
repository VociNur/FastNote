use eframe::egui::Color32;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct DisplayError {
    pub message: String,
    pub color: Color32,
    pub created_at: Instant,
    pub duration: Duration,
}

impl DisplayError {
    /// Crée une nouvelle erreur avec un message et une durée
    pub fn new(message: impl Into<String>, duration: Duration) -> Self {
        Self {
            message: message.into(),
            color: Color32::from_rgb(180, 20, 20), // rouge par défaut
            created_at: Instant::now(),
            duration,
        }
    }

    /// Vérifie si l’erreur est encore valide
    pub fn is_active(&self) -> bool {
        self.created_at.elapsed() < self.duration
    }
}
