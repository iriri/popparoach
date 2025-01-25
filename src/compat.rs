#[doc(hidden)]
#[macro_export]
macro_rules! __cut {
   ($buf:expr, $i:expr, $j:expr$(,)?) => {{
      let buf: &_ = $buf;
      const I: usize = $i;
      const J: usize = $j;
      #[inline(always)]
      fn launder<T>(buf: &[T]) -> &[T; J - I] {
         unsafe { &*(buf.as_ptr() as *const [T; J - I]) }
      }
      launder(&buf[I..J])
   }};
}

pub use __cut as cut;

#[doc(hidden)]
#[macro_export]
macro_rules! __cut_mut {
   ($buf:expr, $i:expr, $j:expr$(,)?) => {{
      let buf: &mut _ = $buf;
      const I: usize = $i;
      const J: usize = $j;
      #[inline(always)]
      const fn launder<T>(buf: &mut [T]) -> &mut [T; J - I] {
         unsafe { &mut *(buf.as_mut_ptr() as *mut [T; J - I]) }
      }
      launder(&mut buf[I..J])
   }};
}

pub use __cut_mut as cut_mut;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn cut() {
      let mut xs = [0, 1, 2, 3, 4, 5];
      assert_eq!(cut!(&xs, 0, 6), &xs);
      assert_eq!(cut!(&xs, 0, 4), &xs[..4]);
      assert_eq!(cut!(cut!(&xs, 1, 5), 0, 4), &xs[1..5][..4]);
      assert_eq!(cut_mut!(&mut xs, 6, 6), &[]);
      assert_eq!(cut_mut!(&mut xs, 5, 6), &[5]);
      assert_eq!(cut!(cut_mut!(&mut xs, 2, 4), 0, 1), &[2]);
      assert_eq!(cut_mut!(cut_mut!(&mut xs, 2, 4), 1, 1), &[]);

      assert_eq!(cut!(&xs[..], 0, 6), &xs);
      assert_eq!(cut!(&xs[..], 0, 4), &xs[..4]);
      assert_eq!(cut!(cut!(&xs[..], 1, 5), 0, 4), &xs[1..5][..4]);
      assert_eq!(cut_mut!(&mut xs[..], 6, 6), &[]);
      assert_eq!(cut_mut!(&mut xs[..], 5, 6), &[5]);
      assert_eq!(cut!(cut_mut!(&mut xs[..], 2, 4), 0, 1), &[2]);
      assert_eq!(cut_mut!(cut_mut!(&mut xs[..], 2, 4), 1, 1), &[]);
   }
}
