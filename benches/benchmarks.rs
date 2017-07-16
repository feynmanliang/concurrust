#![feature(test)]

extern crate rand;
extern crate test;

extern crate concurrust;

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    use test::Bencher;

    #[bench]
    fn bench_map(b: &mut Bencher) {
        b.iter(|| {
            let xs = rand::sample(&mut rand::thread_rng(), 1..100000, 1000);
            concurrust::map(|x| {
                thread::sleep_ms(100);
                x+1
            }, xs)
        });
    }

    #[bench]
    fn bench_parmap(b: &mut Bencher) {
        b.iter(|| {
            let xs = rand::sample(&mut rand::thread_rng(), 1..100000, 5000);
            concurrust::parmap(Arc::new(|x| {
                thread::sleep_ms(100);
                x+1
            }), xs)
        });
    }
}
