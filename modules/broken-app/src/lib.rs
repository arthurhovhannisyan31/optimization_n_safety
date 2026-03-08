use std::ptr;

pub mod algo;
pub mod concurrency;

/// Sum of even values.
/// Here, `get_unchecked` is intentionally used with off-by-one,
/// which causes UB when accessing beyond the slice's bounds.
pub fn sum_even(values: &[i64]) -> i64 {
  let mut acc = 0;
  // Bug 1: off-by-one
  // Iterate withing slice index range
  for idx in 0..values.len() {
    let v = values[idx];
    if v % 2 == 0 {
      acc += v;
    }
  }
  acc
}

/// Counting non-zero bytes. The buffer is intentionally not freed,
/// which will lead to a memory leak (Valgrind will detect this).
pub fn leak_buffer(input: &[u8]) -> usize {
  let boxed = input.to_vec().into_boxed_slice();
  let len = input.len();
  let raw = Box::into_raw(boxed) as *mut u8;

  let mut count = 0;
  unsafe {
    for i in 0..len {
      if *raw.add(i) != 0_u8 {
        count += 1;
      }
    }
    // Bug 3: memory leak
    // Restore proper memory layout for input.len() elements and its deallocation
    let slice_ptr: *mut [u8] = ptr::slice_from_raw_parts_mut(raw, len);
    let _ = Box::from_raw(slice_ptr);
  }
  count
}

/// Loose string normalization: remove spaces and convert to lowercase,
/// but ignore repeated spaces/tabs within the text.
pub fn normalize(input: &str) -> String {
  // Use regex as alternative to split_whitespace()
  let res = input.replace(' ', "").to_lowercase();
  res.replace('\t', "").to_lowercase()
}

/// Logical error: averaging over all elements, although it should only consider positive ones.
/// Dividing by the slice length gives an incorrect result.
pub fn average_positive(values: &[i64]) -> f64 {
  // Bug 2: wrong logic
  // Filter out only positive numbers
  let positives: Vec<i64> = values.iter().copied().filter(|&v| v > 0).collect();
  if positives.is_empty() {
    return 0.0;
  }
  let sum: i64 = positives.iter().sum();
  sum as f64 / positives.len() as f64
}

/// Use-after-free: returns the value after the box is freed.
/// UB, will appear under ASan/Miri.

pub unsafe fn use_after_free() -> i32 {
  let b = Box::new(42_i32);
  let raw = Box::into_raw(b);
  let res = unsafe { *raw + *raw };
  // Bug 5
  // Swap drop and result calculation to prevent UAF
  unsafe {
    drop(Box::from_raw(raw));
  }
  res
}
