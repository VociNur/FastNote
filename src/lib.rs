// #![warn(clippy::all, rust_2018_idioms)]

use std::{
    fs::{self, File},
    path::PathBuf,
};

use chrono::{Local, NaiveDate};
use eframe::egui::{self, Color32};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

use crate::paths::{PROJECT_DEFAULT_FOLDER, SAVE_DEFAULT_FOLDER};

// use egui::Color32;
pub mod app;

fn rect_points_to_pixels(rect_points: egui::Rect, ppp: f32) -> egui::Rect {
    egui::Rect::from_min_max(rect_points.min * ppp, rect_points.max * ppp)
}

pub fn folder_exists(parent: &PathBuf, name: &str) -> bool {
    parent.join(name).exists()
}
