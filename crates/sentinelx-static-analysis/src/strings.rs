use regex::Regex;
use std::sync::LazyLock;

static URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://[^\s\x00-\x1f\"<>]+").unwrap());

static API_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(VirtualAlloc|WriteProcessMemory|CreateRemoteThread|RegSetValue|WinExec|ShellExecute|curl|wget|powershell|cmd\.exe|/bin/sh|chmod \+x)").unwrap()
});

pub fn extract_printable_strings(data: &[u8], min_len: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current = String::new();

    for &byte in data {
        if (32..=126).contains(&byte) || byte == b'\t' {
            current.push(byte as char);
        } else if current.len() >= min_len {
            strings.push(std::mem::take(&mut current));
        } else {
            current.clear();
        }
    }
    if current.len() >= min_len {
        strings.push(current);
    }
    strings
}

pub fn extract_urls(strings: &[String]) -> Vec<String> {
    let mut urls = Vec::new();
    for s in strings {
        for cap in URL_RE.find_iter(s) {
            urls.push(cap.as_str().to_string());
        }
    }
    urls.sort();
    urls.dedup();
    urls
}

pub fn extract_suspicious_apis(strings: &[String]) -> Vec<String> {
    let mut apis = Vec::new();
    for s in strings {
        for cap in API_RE.find_iter(s) {
            apis.push(cap.as_str().to_string());
        }
    }
    apis.sort();
    apis.dedup();
    apis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_urls() {
        let strings = vec!["visit http://evil.example/payload".into()];
        assert_eq!(extract_urls(&strings).len(), 1);
    }
}
