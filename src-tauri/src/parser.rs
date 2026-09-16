use std::fs;

#[derive(Debug)]
pub struct Entry {
    pub date: String,
    pub name: String,
    pub category: String,
    pub difficulty: String,
    pub approach: String,
    pub complexity: String,
}

/// Danh sách category hợp lệ đã biết — dùng để phát hiện dòng bị lệch cột.
/// Cập nhật danh sách này khi bạn thêm category mới (vd two_pointers, sliding_window...).
const KNOWN_CATEGORIES: &[&str] = &[
    "arrays_hashing",
    "two_pointers",
    "sliding_window",
    "stack",
    "binary_search",
    "linked_list",
    "trees",
    "heap_priority_queue",
    "backtracking",
    "graphs",
    "dynamic_programming",
];

const VALID_DIFFICULTIES: &[&str] = &["Easy", "Medium", "Hard"];

pub fn parse_readme(path: &str) -> Vec<Entry> {
    let content = fs::read_to_string(path)
        .expect("Failed to read README.md");

    let mut entries = Vec::new();
    let mut inside_rows = false;

    for (line_num, line) in content.lines().enumerate() {
        if line.contains("<!-- ROWS -->") {
            inside_rows = true;
            continue;
        }

        if !inside_rows {
            continue;
        }

        if !line.starts_with('|') {
            continue;
        }

        let columns: Vec<&str> = line
            .split('|')
            .map(str::trim)
            .collect();

        if columns.len() < 8 {
            continue;
        }

        // Bỏ header/separator nếu sau này chúng xuất hiện
        if columns[1] == "#" || columns[1].starts_with("---") {
            continue;
        }

        let entry = Entry {
            date: columns[2].to_string(),
            name: clean_name(columns[3]),
            category: columns[4].to_string(),
            difficulty: columns[5].to_string(),
            approach: columns[6].to_string(),
            complexity: columns[7].to_string(),
        };

        // Validate: cảnh báo nếu dữ liệu có dấu hiệu bị lệch cột (do gõ tay sai)
        // Dòng số hiển thị ở đây là số dòng trong file .md, giúp dễ tìm lại để sửa.
        if !KNOWN_CATEGORIES.contains(&entry.category.as_str()) {
            eprintln!(
                "⚠️  Dòng {}: category '{}' không nằm trong danh sách đã biết — \
                nghi ngờ bị lệch cột (Category <-> Cách giải)? Entry: '{}'",
                line_num + 1,
                entry.category,
                entry.name,
            );
        }

        if !VALID_DIFFICULTIES.contains(&entry.difficulty.as_str()) {
            eprintln!(
                "⚠️  Dòng {}: độ khó '{}' không hợp lệ (phải là Easy/Medium/Hard) — \
                nghi ngờ lệch cột. Entry: '{}'",
                line_num + 1,
                entry.difficulty,
                entry.name,
            );
        }

        entries.push(entry);
    }

    entries
}

fn clean_name(name: &str) -> String {
    // Cắt cú pháp markdown link [text](url) -> chỉ giữ "text"
    let base = if let Some(start) = name.find('[') {
        if let Some(end) = name.find(']') {
            &name[start + 1..end]
        } else {
            name
        }
    } else {
        name
    };

    base.replace(" (Update Approach)", "")
}