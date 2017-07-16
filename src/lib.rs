fn map<F,T,U>(f: F, xs: T ) -> Vec<U>
    where F: Fn(T::Item) -> U,
          T: IntoIterator {
    let mut ys = Vec::new();
    for x in xs {
        ys.push(f(x));
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
}
