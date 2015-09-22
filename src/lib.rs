#![feature(test)]

#[cfg(test)]
extern crate test;

/// Makes room for up to ```max``` bytes in the Vector
/// and calls ```f``` with the extent. The number of
/// bytes appended to the Vec is returned by ```f```. 
#[inline]
pub fn append_bytes_uninit_flex<F>(vec: &mut Vec<u8>, max: usize, mut f: F)
                              where F: FnMut(&mut[u8]) -> usize {
    let orig_len = vec.len();
    vec.reserve(max);
    unsafe { vec.set_len(orig_len + max); }
    let n = f(&mut vec[orig_len..]);
    assert!(n <= max);
    unsafe { vec.set_len(orig_len + n); }
}

/// Extends ```vec``` for ```len``` new bytes
/// and calls ```f``` with the extent.
#[inline]
pub fn append_bytes_uninit<F>(vec: &mut Vec<u8>, len: usize, mut f: F)
                              where F: FnMut(&mut[u8]) {
    let orig_len = vec.len();
    vec.reserve(len);
    unsafe { vec.set_len(orig_len + len); }
    f(&mut vec[orig_len..]);
}

#[cfg(test)]
const N: usize = 1_000_000;

#[bench]
fn bench_append_bytes_using_push(b: &mut test::Bencher) {
    let mut vec = Vec::with_capacity(N);
    b.iter(|| {
        let n = test::black_box(N);
        for _ in (0..n) {
            vec.push(123);
        }
    });
}

#[bench]
fn bench_append_bytes_via_append(b: &mut test::Bencher) {
    let mut vec = Vec::with_capacity(N);
    b.iter(|| {
        append_bytes_uninit(&mut vec, test::black_box(N), |mut s| {
                for c in s.iter_mut() {
                        *c = 123;
                }
        });
    });
}
