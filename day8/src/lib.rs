pub mod my_array2d{
    pub struct Array2D<T>{
        data: Vec<T>,
        size: (usize,usize)
    }
    impl<T> Array2D<T> {
        // pub fn from_rows<T: Clone>(rows: &[&[T]]) -> Option<Self> {
        //     let height = rows.len();
        //     if height

        // }
        pub fn len(&self) -> (usize, usize) {self.size}
        pub fn row_len(&self) -> usize {self.size.0}
        pub fn column_len(&self) -> usize {self.size.1}
    }
}