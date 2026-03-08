use std::ptr;

pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
  let mut acc = 0;
  unsafe {
    // Bug 1: off-by-one
    // Iterate withing slice index range
    for idx in 0..values.len() {
      let v = *values.get_unchecked(idx);
      if v % 2 == 0 {
        acc += v;
      }
    }
  }
  acc
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
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

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
  // Use regex as alternative to split_whitespace()
  let res = input.replace(' ', "").to_lowercase();
  res.replace('\t', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
  // Bug 2: wrong logic
  // Filter out only positive numbers
  let positives: Vec<i64> =
    values.iter().copied().filter(|&v| v >= 0).collect();
  let sum: i64 = positives.iter().filter(|&v| v >= &0).sum();
  if positives.is_empty() {
    return 0.0;
  }
  sum as f64 / positives.len() as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.

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
