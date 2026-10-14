// Author: Daniel Hallman

use clap::Parser;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

const QUERY_KEYS: &[&str] = &["q", "query", "search_query", "text"];

#[derive(Parser, Debug)]
#[command(
    name = "query-miner",
    about = "Extract and deduplicate nested `queries` from ChatGPT response data."
)]
struct Cli {
    #[arg(help = "Input path or - for stdin", default_value = "-")]
    input: String,

    #[arg(
        long,
        value_parser = ["text", "markdown", "json", "csv"],
        default_value = "text",
        help = "Output format"
    )]
    format: String,
}

pub fn parse_documents(raw: &str) -> Vec<Value> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
        return vec![v];
    }

    let mut docs = Vec::new();
    for line in trimmed.lines() {
        let mut candidate = line.trim();
        if let Some(rest) = candidate.strip_prefix("data:") {
            candidate = rest.trim();
        }
        if candidate.is_empty() || candidate == "[DONE]" {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(candidate) {
            docs.push(v);
        }
    }

    if !docs.is_empty() {
        return docs;
    }

    // Try finding "queries": patterns
    if let Ok(re) = Regex::new(r#"(?i)["']queries["']\s*:"#) {
        for m in re.find_iter(trimmed) {
            let start = m.end();
            let slice = trimmed[start..].trim_start();
            let mut de = serde_json::Deserializer::from_str(slice);
            if let Ok(v) = Value::deserialize(&mut de) {
                docs.push(serde_json::json!({ "queries": v }));
            }
        }
    }

    docs
}

pub fn strings_from_query_value(val: &Value, out: &mut Vec<String>) {
    match val {
        Value::String(s) => out.push(s.clone()),
        Value::Array(arr) => {
            for item in arr {
                strings_from_query_value(item, out);
            }
        }
        Value::Object(map) => {
            let mut matched = false;
            for key in QUERY_KEYS {
                if let Some(v) = map.get(*key) {
                    strings_from_query_value(v, out);
                    matched = true;
                }
            }
            if !matched {
                for v in map.values() {
                    strings_from_query_value(v, out);
                }
            }
        }
        _ => {}
    }
}

pub fn find_queries(val: &Value, out: &mut Vec<String>) {
    match val {
        Value::Array(arr) => {
            for item in arr {
                find_queries(item, out);
            }
        }
        Value::Object(map) => {
            for (key, item) in map {
                if key.eq_ignore_ascii_case("queries") {
                    strings_from_query_value(item, out);
                } else {
                    find_queries(item, out);
                }
            }
        }
        _ => {}
    }
}

pub fn find_direct_query_value(val: &Value, out: &mut Vec<String>) {
    match val {
        Value::String(s) => out.push(s.clone()),
        Value::Object(map) if map.keys().any(|k| QUERY_KEYS.contains(&k.as_str())) => {
            strings_from_query_value(val, out);
        }
        Value::Array(arr)
            if arr.iter().all(|item| {
                item.is_string()
                    || (item.is_object()
                        && item
                            .as_object()
                            .is_some_and(|m| m.keys().any(|k| QUERY_KEYS.contains(&k.as_str()))))
            }) =>
        {
            strings_from_query_value(val, out);
        }
        _ => {}
    }
}

pub fn normalize_queries(values: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    let mut seen = HashSet::new();

    for val in values {
        let words: Vec<&str> = val.split_whitespace().collect();
        let query = words.join(" ");
        let trimmed = query.trim();
        if trimmed.is_empty() || trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            continue;
        }
        let fingerprint = trimmed.to_lowercase();
        if seen.insert(fingerprint) {
            output.push(trimmed.to_string());
        }
    }

    output
}

pub fn render(queries: &[String], format: &str) -> String {
    match format {
        "json" => format!(
            "{}\n",
            serde_json::to_string_pretty(queries).unwrap_or_else(|_| "[]".to_string())
        ),
        "markdown" => {
            let mut buf = String::new();
            for q in queries {
                buf.push_str(&format!("- {}\n", q));
            }
            buf
        }
        "csv" => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            wtr.write_record(["query"]).unwrap();
            for q in queries {
                wtr.write_record([q]).unwrap();
            }
            String::from_utf8(wtr.into_inner().unwrap()).unwrap()
        }
        _ => {
            let mut buf = String::new();
            for q in queries {
                buf.push_str(&format!("{}\n", q));
            }
            buf
        }
    }
}

pub fn extract_queries_from_raw(raw: &str) -> Vec<String> {
    let docs = parse_documents(raw);
    let mut extracted = Vec::new();
    for d in &docs {
        find_queries(d, &mut extracted);
    }
    if extracted.is_empty() && docs.len() == 1 {
        find_direct_query_value(&docs[0], &mut extracted);
    }
    normalize_queries(&extracted)
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let raw = if cli.input == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(PathBuf::from(&cli.input))?
    };

    let queries = extract_queries_from_raw(&raw);
    if queries.is_empty() {
        eprintln!("No queries found. Copy a response body that contains a `queries` field.");
        std::process::exit(1);
    }

    print!("{}", render(&queries, &cli.format));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_queries_json() {
        let json_text = r#"{
            "queries": [
                "mac mini m4",
                "  Mac   Mini M4 ",
                {"query": "mac studio deal"},
                {"q": "apple refurb store"},
                {"search_query": "m4 pro benchmark"},
                {"text": "mac mini ports"},
                "https://example.com/not-a-query",
                "http://example.com/also-not-a-query",
                "   "
            ]
        }"#;
        let norm = extract_queries_from_raw(json_text);
        assert_eq!(
            norm,
            vec![
                "mac mini m4",
                "mac studio deal",
                "apple refurb store",
                "m4 pro benchmark",
                "mac mini ports"
            ]
        );
    }

    #[test]
    fn test_extract_queries_sse_and_jsonl() {
        let sse_text = "data: {\"queries\": [\"test query 1\"]}\n{\"nested\": {\"Queries\": [\"test query 2\"]}}\n\ndata: [DONE]\n";
        let norm = extract_queries_from_raw(sse_text);
        assert_eq!(norm, vec!["test query 1", "test query 2"]);
    }

    #[test]
    fn test_extract_embedded_queries_fragment() {
        let fragment = r#"event: delta
some_prefix "queries": ["embedded one", {"query": "embedded two"}], trailing_tokens"#;
        let norm = extract_queries_from_raw(fragment);
        assert_eq!(norm, vec!["embedded one", "embedded two"]);
    }

    #[test]
    fn test_direct_query_value_and_arbitrary_json_rejection() {
        let direct_list = r#"["direct one", {"q": "direct two"}]"#;
        assert_eq!(
            extract_queries_from_raw(direct_list),
            vec!["direct one", "direct two"]
        );

        let direct_obj = r#"{"search_query": "single object query"}"#;
        assert_eq!(
            extract_queries_from_raw(direct_obj),
            vec!["single object query"]
        );

        let unrelated = r#"{"status": "ok", "items": [{"id": 1, "name": "not a query"}]}"#;
        assert!(extract_queries_from_raw(unrelated).is_empty());
    }

    #[test]
    fn test_render_formats() {
        let queries = vec!["q1".to_string(), "q2".to_string()];
        let md = render(&queries, "markdown");
        assert_eq!(md, "- q1\n- q2\n");

        let txt = render(&queries, "text");
        assert_eq!(txt, "q1\nq2\n");

        let json_out = render(&queries, "json");
        let parsed: Vec<String> = serde_json::from_str(&json_out).unwrap();
        assert_eq!(parsed, queries);

        let csv_out = render(&queries, "csv");
        assert!(csv_out.contains("query\nq1\nq2\n"));
    }
}
