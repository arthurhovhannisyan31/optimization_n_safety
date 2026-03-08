pub mod algo;
pub mod concurrency;

/// Safe and fast version: no `unsafe`, pure slice pass.
pub fn sum_even(values: &[i64]) -> i64 {
  values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Counting non-zero bytes without leaks.
pub fn leak_buffer(input: &[u8]) -> usize {
  input.iter().filter(|b| **b != 0).count()
}

/// Normalization: remove all types of whitespace characters and convert to lowercase.
pub fn normalize(input: &str) -> String {
  input.split_whitespace().collect::<String>().to_lowercase()
}

/// Correct averaging of only positive numbers.
pub fn average_positive(values: &[i64]) -> f64 {
  let positives: Vec<i64> = values.iter().copied().filter(|v| *v > 0).collect();
  if positives.is_empty() {
    return 0.0;
  }
  let sum: i64 = positives.iter().sum();
  sum as f64 / positives.len() as f64
}
