use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// Небезопасный инкремент через несколько потоков.
/// Использует global static mut без синхронизации — data race.
/// Bug 6: Use Arc<RwLock> as alternative to Atomic_int
pub fn race_increment(
  counter: Arc<RwLock<u64>>,
  iterations: usize,
  threads: usize,
) -> u64 {
  {
    let counter = Arc::clone(&counter);
    let mut counter_write =
      counter.write().expect("Failed locking counter for write");
    *counter_write = 0;
  }
  let mut handles = Vec::new();
  for _ in 0..threads {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
      for _ in 0..iterations {
        let mut counter_write =
          counter.write().expect("Failed locking counter for write");
        *counter_write += 1;
      }
    }));
  }
  for h in handles {
    let _ = h.join();
  }
  *counter.read().expect("Failed reading ")
}

/// Плохая «синхронизация» — просто sleep, возвращает потенциально устаревшее значение.
pub fn read_after_sleep(counter: Arc<RwLock<u64>>) -> u64 {
  thread::sleep(Duration::from_millis(10));
  *counter.read().expect("Failed reading ")
}

/// Сброс счётчика (также небезопасен, без синхронизации).
pub fn reset_counter(counter: Arc<RwLock<u64>>) {
  let mut counter_write =
    counter.write().expect("Failed locking counter for write");
  *counter_write = 0;
}
