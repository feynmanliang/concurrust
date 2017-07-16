pub fn map<F,T,U>(f: F, xs: T ) -> Vec<U>
    where F: Fn(T::Item) -> U,
          T: IntoIterator {
    let mut ys = Vec::new();
    for x in xs {
        ys.push(f(x));
    }
    ys
}

pub fn reduce<F,T,A>(f: F, a0: A, xs: T) -> A
    where F: Fn(T::Item, A) -> A,
          T: IntoIterator {
    let mut a = a0;
    for x in xs {
        a = f(x, a);
    }
    a
}

pub fn scan<F,T,A>(f: F, a0: A, xs: T) -> Vec<A>
    where F: Fn(T::Item, A) -> A,
          A: Copy,
          T: IntoIterator {
    let mut ys = vec![a0];
    for x in xs {
        let a_prev = ys[ys.len()-1];
        ys.push(f(x, a_prev));
    }
    ys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_identity() {
        let ys = map(|x| x, vec![1,2,3]);
        assert_eq!(ys, vec![1,2,3])
    }

    #[test]
    fn map_plus_one() {
        let ys = map(|x| x+1, vec![1,2,3]);
        assert_eq!(ys, vec![2,3,4])
    }

    #[test]
    fn reduce_identity() {
        let ys = reduce(|x, mut a| {
            a.push(x);
            a
        }, vec![], vec![1,2,3]);
        assert_eq!(ys, vec![1,2,3])
    }

    #[test]
    fn reduce_sum() {
        let a = reduce(|x, a| a+x, 0, vec![1,2,3]);
        assert_eq!(a, 6)
    }

    #[test]
    fn scan_identity() {
        let a = scan(|x, a| x, 0, vec![1,2,3]);
        assert_eq!(a, [0,1,2,3])
    }

    #[test]
    fn scan_sum() {
        let a = scan(|x, a| a+x, 0, vec![1,2,3]);
        assert_eq!(a, [0,1,3,6])
    }
}
