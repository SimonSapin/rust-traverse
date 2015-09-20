use std::mem;
use Traversal;

impl<'a, T> Traversal for &'a [T] {
    type Item = &'a T;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(&'a T) -> bool {
        unsafe {
            let ptr = self.as_ptr();
            let len = self.len();

            let is_zero_size = mem::size_of::<T>() == 0;

            if is_zero_size {
                for _ in 0..len {
                    // Just give some pointer, doesn't matter what.
                    if f(mem::transmute(1usize)) { break }
                }
            } else {
                let mut current = ptr;
                let end = ptr.offset(len as isize);
                while current != end {
                    if f(mem::transmute(current)) { break }
                    current = current.offset(1);
                }
            }
        }
    }
}

impl<'a, T> Traversal for &'a mut [T] {
    type Item = &'a mut T;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(&'a mut T) -> bool {
        unsafe {
            let ptr = self.as_mut_ptr();
            let len = self.len();

            let is_zero_size = mem::size_of::<T>() == 0;

            if is_zero_size {
                for _ in 0..len {
                    // Just give some pointer, doesn't matter what.
                    if f(mem::transmute(1usize)) { break }
                }
            } else {
                let mut current = ptr;
                let end = ptr.offset(len as isize);
                while current != end {
                    if f(mem::transmute(current)) { break }
                    current = current.offset(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use Traversal;

    #[test]
    fn test_basic() {
        let data = [1, 2, 5, 4, 6, 7];
        let traversal: Vec<usize> = data.map(|&x| x).collect();
        assert_eq!(traversal, data);
    }

    #[test]
    fn test_zero_size() {
        let data = [(), (), ()];
        let traversal: Vec<()> = data.map(|&x| x).collect();
        assert_eq!(traversal, data);
    }
}

#[cfg(all(test, feature = "nightly"))]
mod bench {

    use Traversal;
    use test::Bencher;

    #[bench]
    fn bench_internal (bench: &mut Bencher) {
        use rand::random;

        let data: Vec<usize> = (0..10000).map(|_| random()).collect();
        bench.iter(|| {
            data.run(|x| { ::test::black_box(x); });
        });
    }

    #[bench]
    fn bench_external (bench: &mut Bencher) {
        use rand::random;

        let data: Vec<usize> = (0..10000).map(|_| random()).collect();
        bench.iter(|| {
            for datum in data.iter() {
                ::test::black_box(datum);
            }
        });
    }
}
