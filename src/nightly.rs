pub trait Cut<T> {
   const N: usize = usize::MAX;

   fn cut<const I: usize, const J: usize>(&self) -> &[T; J - I]
   where
      [(); (J <= Self::N) as usize - 1]:;

   fn cut_mut<const I: usize, const J: usize>(&mut self) -> &mut [T; J - I]
   where
      [(); (J <= Self::N) as usize - 1]:;
}

impl<T, const N: usize> Cut<T> for [T; N] {
   const N: usize = N;

   #[inline(always)]
   fn cut<const I: usize, const J: usize>(&self) -> &[T; J - I] {
      unsafe { &*(self.as_ptr().add(I) as *const [T; J - I]) }
   }

   #[inline(always)]
   fn cut_mut<const I: usize, const J: usize>(&mut self) -> &mut [T; J - I] {
      unsafe { &mut *(self.as_mut_ptr().add(I) as *mut [T; J - I]) }
   }
}

#[cold]
#[track_caller]
#[inline(never)]
fn panic_oob() -> ! {
   panic!("end index out of bounds");
}

impl<T> Cut<T> for [T] {
   #[inline]
   fn cut<const I: usize, const J: usize>(&self) -> &[T; J - I] {
      if J > self.len() {
         panic_oob();
      }
      unsafe { &*(self.as_ptr().add(I) as *const [T; J - I]) }
   }

   #[inline]
   fn cut_mut<const I: usize, const J: usize>(&mut self) -> &mut [T; J - I] {
      if J > self.len() {
         panic_oob();
      }
      unsafe { &mut *(self.as_mut_ptr().add(I) as *mut [T; J - I]) }
   }
}

pub trait CutAt<T, const N: usize> {
   fn cut_at<const I: usize>(&self) -> (&[T; I], &[T; N - I]);
   fn cut_at_mut<const I: usize>(&mut self) -> (&mut [T; I], &mut [T; N - I]);
}

impl<T, const N: usize> CutAt<T, N> for [T; N] {
   #[inline(always)]
   fn cut_at<const I: usize>(&self) -> (&[T; I], &[T; N - I]) {
      let l = self.as_ptr() as *const [T; I];
      unsafe { (&*l, &*(self.as_ptr().add(I) as *const [T; N - I])) }
   }

   #[inline(always)]
   fn cut_at_mut<const I: usize>(&mut self) -> (&mut [T; I], &mut [T; N - I]) {
      let l = self.as_mut_ptr() as *mut [T; I];
      unsafe { (&mut *l, &mut *(self.as_mut_ptr().add(I) as *mut [T; N - I])) }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn cut() {
      let mut xs = [0, 1, 2, 3, 4, 5];
      assert_eq!(xs.cut::<0, 6>(), &xs);
      assert_eq!(xs.cut::<0, 4>(), &xs[..4]);
      assert_eq!(xs.cut::<1, 5>().cut::<0, 4>(), &xs[1..5][..4]);
      assert_eq!(xs.cut_mut::<6, 6>(), &[]);
      assert_eq!(xs.cut_mut::<5, 6>(), &[5]);
      assert_eq!(xs.cut_mut::<2, 4>().cut::<0, 1>(), &[2]);
      assert_eq!(xs.cut_mut::<2, 4>().cut_mut::<1, 1>(), &[]);

      assert_eq!(xs[..].cut::<0, 6>(), &xs);
      assert_eq!(xs[..].cut::<0, 4>(), &xs[..4]);
      assert_eq!(xs[..].cut::<1, 5>().cut::<0, 4>(), &xs[1..5][..4]);
      assert_eq!(xs[..].cut_mut::<6, 6>(), &[]);
      assert_eq!(xs[..].cut_mut::<5, 6>(), &[5]);
      assert_eq!(xs[..].cut_mut::<2, 4>().cut::<0, 1>(), &[2]);
      assert_eq!(xs[..].cut_mut::<2, 4>().cut_mut::<1, 1>(), &[]);

      assert_eq!(xs.cut_at::<0>(), (&[], &xs));
      assert_eq!(xs.cut_at::<1>(), (&[0], &[1, 2, 3, 4, 5]));
      assert_eq!(xs.cut_at_mut::<2>(), (&mut [0, 1], &mut [2, 3, 4, 5]));
   }
}
