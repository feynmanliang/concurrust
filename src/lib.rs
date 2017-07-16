struct Task<T: Copy> {
    closure: Box<Fn() -> T>,
    pub result: Option<T>
}

impl <T: Copy> Task<T> {
    fn run(&self) -> T {
        (self.closure)()
    }

    pub fn await(&mut self) -> T {
        if let Some(res) = self.result {
            res
        } else {
            let res = self.run();
            self.result = Some(res);
            res
        }
    }
}



/// Example:
///
/// ```
/// use concurrust::*;
/// let ys = map(|x| x+1, vec![1,2,3]);
/// assert_eq!(ys, vec![2,3,4])
/// ```
pub fn map<F,T,U>(f: F, xs: T ) -> Vec<U>
    where F: Fn(T::Item) -> U,
          T: IntoIterator {
    let mut ys = Vec::new();
    for x in xs {
        ys.push(f(x));
    }
    ys
}

/// Example:
///
/// ```
/// use concurrust::*;
/// let a = scan(|x, a| a+x, 0, vec![1,2,3]);
/// assert_eq!(a, [0,1,3,6])
/// ```
pub fn reduce<F,T,A>(f: F, a0: A, xs: T) -> A
    where F: Fn(T::Item, A) -> A,
          T: IntoIterator {
    let mut a = a0;
    for x in xs {
        a = f(x, a);
    }
    a
}

/// Example:
///
/// ```
/// use concurrust::*;
/// let a = reduce(|x, a| a+x, 0, vec![1,2,3]);
/// assert_eq!(a, 6)
/// ```
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
mod tests {}
