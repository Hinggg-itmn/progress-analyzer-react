use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Report {
    pub total: usize,
    pub progress_percent: f64,
    pub average_gap: f64,
    pub by_category: Vec<(String, usize)>,
    pub by_difficulty: Vec<(String, usize)>,
    pub top_approaches: Vec<(String, usize)>,
}

pub fn generate_report(
    category_stats: &HashMap<String, usize>,
    difficulty_stats: &HashMap<String, usize>,
    approach_stats: &HashMap<String, usize>,
    average_gap: f64,
) -> Report {
    let total = category_stats.values().sum::<usize>();

    let mut by_category: Vec<_> = category_stats.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    by_category.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let difficulty_order = ["Easy", "Medium", "Hard"];
    let by_difficulty: Vec<_> = difficulty_order.iter()
        .filter_map(|d| difficulty_stats.get(*d).map(|c| (d.to_string(), *c)))
        .collect();

    let mut top_approaches: Vec<_> = approach_stats.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    top_approaches.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    top_approaches.truncate(5);

    Report {
        total,
        progress_percent: (total as f64 / 150.0) * 100.0,
        average_gap,
        by_category,
        by_difficulty,
        top_approaches,
    }
}