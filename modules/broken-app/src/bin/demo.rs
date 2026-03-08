use broken_app::{algo, leak_buffer, normalize, sum_even, use_after_free};

fn main() {
  sum_even(&[1, 2, 3, 4]);
  leak_buffer(&[1_u8, 0, 2, 3]);
  normalize(" Hello World ");
  algo::slow_fib(20);
  algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
  unsafe { use_after_free() };
}
