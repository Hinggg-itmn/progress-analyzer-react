mod parser;
mod report;
mod stats;

use report::Report;

#[tauri::command]
fn analyze_notes(path: String) -> Result<Report, String> {
    let entries = parser::parse_readme(&path);

    let category_stats = stats::count_by_category(&entries);
    let difficulty_stats = stats::count_by_difficulty(&entries);
    let approach_stats = stats::count_approaches(&entries);
    let average_gap = stats::calculate_average_gap(&entries);

    Ok(report::generate_report(
        &category_stats,
        &difficulty_stats,
        &approach_stats,
        average_gap,
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}