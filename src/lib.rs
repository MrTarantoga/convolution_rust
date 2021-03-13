#[warn(dead_code)]
mod convolution {
    use std::iter::Sum;
    use std::ops::{Add, Div, Mul, Sub};

    pub struct DataSet<T: Add + Sub + Mul + Div>{
        function_a: Vec<T>,
        function_b: Vec<T>,
        state: usize
    }
    impl<T> DataSet<T> where T: Add + Sub + Mul + Div + Clone {
        pub fn new(function_a: &[T], function_b: &[T]) -> DataSet<T> {
            DataSet {
                function_a: function_a.to_vec(),
                function_b: function_b.to_vec(),
                state: 1
            }
        }
    }

    impl<T> Iterator for DataSet<T>
        where
            T: Add + Sub + Mul + Div + Sum,
            for<'a> &'a T: Mul<Output = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.state == 1 {
                self.function_b.reverse();
            }
            if self.state <= (self.function_a.len() + self.function_b.len() - 1) {
                let _slice_func_a = if self.state < self.function_a.len() {
                    &self.function_a[..self.state]
                } else if self.state == (self.function_a.len()) {
                    &self.function_a[..]
                } else {
                    &self.function_a[(self.state - self.function_a.len())..]
                };

                let _slice_func_b = if self.state < self.function_b.len() {
                    &self.function_b[(self.function_b.len() - self.state)..]
                } else if self.state == (self.function_b.len()) {
                    &self.function_b[..]
                } else {
                    &self.function_b[..(self.state - self.function_b.len() + 1)]
                };
                self.state += 1;
                Some(
                    _slice_func_a
                        .iter()
                        .zip(_slice_func_b.iter())
                        .map(|x| x.0 * x.1)
                        .sum()
                )
            } else {
              None
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::convolution::DataSet;

    #[test]
    fn iterator_test_next(){
        let mut obj = DataSet::new(
            &[1, 2, 3],
            &[1, 2, 3],
        );
        assert_eq!(1, obj.next().unwrap());
        assert_eq!(4, obj.next().unwrap());
        assert_eq!(10, obj.next().unwrap());
        assert_eq!(12, obj.next().unwrap());
        assert_eq!(9, obj.next().unwrap());
        assert_eq!(None, obj.next());

    }
    #[test]
    fn iterator_test_last(){
        let obj = DataSet::new(
            &[1, 2, 3],
            &[1, 2, 3],
        );
        assert_eq!(9, obj.last().unwrap())

    }

    #[test]
    fn iterator_test_sum(){
        let obj = DataSet::new(
            &[1, 2, 3],
            &[1, 2, 3],
        );
        assert_eq!(36, obj.sum())

    }

    #[test]
    fn iterator_test_float(){
        let mut obj = DataSet::new(
            &[1.5, 90.87, -13.0],
            &[1.5, -9.88, 13.9],

        );
        print!("{:?}\n", obj.next());
    }
}
