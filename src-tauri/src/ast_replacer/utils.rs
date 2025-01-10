use serde_json::Value;

pub fn get_line_number(source_text: &str, offset: usize) -> usize {
    // Split by lines and calculate cumulative byte lengths to pinpoint the correct line
    let mut cumulative_offset = 0;

    for (i, line) in source_text.lines().enumerate() {
        cumulative_offset += line.len() + 1; // Add 1 for the newline character
        if offset < cumulative_offset {
            return i + 1; // Lines are 1-indexed
        }
    }

    // Fallback: If offset exceeds the total length
    source_text.lines().count()
}

pub fn transform_to_result(mut debug_results: Vec<Value>) -> String {
    let mut result = String::new();
    let mut current_line = 0; // Tracks the last line position


    // Sort by line
    debug_results.sort_by(|a, b| {
        let line_a = a.get("line").and_then(|l| l.as_u64()).unwrap_or(0);
        let line_b = b.get("line").and_then(|l| l.as_u64()).unwrap_or(0);
        line_a.cmp(&line_b)
    });

    for (i, item) in debug_results.iter().enumerate() {
        if let Some(line) = item.get("line").and_then(|l| l.as_u64()) {
            let target_line = line as usize;

            // Add exact newlines to move to the correct line
            while current_line < target_line - 1 {
                result.push('\n');
                current_line += 1;
            }

            if current_line == target_line && !result.is_empty() {
                result.push(' ');
            }

            // Append the value if it exists
            if let Some(value) = item.get("value") {
                let value_str = value_to_string(value);
                result.push_str(&value_str);

                // Only add a newline if it's the last value on this line
                if i + 1 < debug_results.len() {
                    if let Some(next_line) = debug_results[i + 1].get("line").and_then(|l| l.as_u64()) {
                        if next_line as usize != target_line {
                            result.push('\n');
                        }
                    }
                } else {
                    result.push('\n'); // Final value, ensure newline
                }
            }

            current_line = target_line; // Update the current line after appending value
        }
    }

    result
}

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => format!(
            "[{}]",
            arr.iter()
                .map(value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Object(obj) => format!(
            "{{{}}}",
            obj.iter()
                .map(|(k, v)| format!("\"{}\": {}", k, value_to_string(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}