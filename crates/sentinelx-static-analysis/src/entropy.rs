pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    let len = data.len() as f64;
    freq.iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

pub fn is_likely_packed(entropy: f64) -> bool {
    entropy > 7.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_data_has_high_entropy() {
        let data: Vec<u8> = (0..=255).collect();
        assert!(shannon_entropy(&data) > 7.9);
    }

    #[test]
    fn repeated_byte_has_low_entropy() {
        let data = vec![0u8; 1000];
        assert!(shannon_entropy(&data) < 0.01);
    }
}
