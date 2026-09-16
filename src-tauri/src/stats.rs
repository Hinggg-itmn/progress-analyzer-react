use std::collections::HashMap;
use crate::parser::Entry;

/// Đếm số bài theo category
pub fn count_by_category(entries: &[Entry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.category.clone()).or_insert(0) += 1;
    }
    counts
}

/// Đếm số bài theo độ khó
pub fn count_by_difficulty(entries: &[Entry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.difficulty.clone()).or_insert(0) += 1;
    }
    counts
}

/// Đếm số lần dùng mỗi approach
pub fn count_approaches(entries: &[Entry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.approach.clone()).or_insert(0) += 1;
    }
    counts
}

/// Tính khoảng cách ngày trung bình giữa các bài (ngày)
pub fn calculate_average_gap(entries: &[Entry]) -> f64 {
    if entries.len() < 2 {
        return 0.0;
    }

    // Parse dates (format: YYYY-MM-DD)
    let mut dates: Vec<_> = entries
        .iter()
        .filter_map(|e| chrono::NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok())
        .collect();

    // Sort chronologically
    dates.sort();

    if dates.len() < 2 {
        return 0.0;
    }

    // Tính tổng gap
    let mut total_gap = 0i64;
    for i in 1..dates.len() {
        let gap = (dates[i] - dates[i - 1]).num_days();
        total_gap += gap;
    }

    total_gap as f64 / (dates.len() - 1) as f64
}