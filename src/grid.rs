use std::{
    ops::{Index, IndexMut},
    vec::Drain,
};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    column_count: usize,
}
impl<T> Grid<T> {
    #[inline]
    pub fn new(column_count: usize) -> Self {
        Self {
            data: Vec::with_capacity(column_count),
            column_count,
        }
    }
    #[inline]
    pub fn from_parts(data: Vec<T>, column_count: usize) -> Option<Self> {
        if data.len() % column_count == 0 {
            Some(Self { data, column_count })
        } else {
            None
        }
    }
    pub fn remove_column(&mut self, column:usize) {
        if column < self.column_count {
            for index in (column..self.data.len()).step_by(column).rev() {
                self.data.remove(index);
            }
            self.column_count-=1;
        }
    }
    pub fn remove_row(&mut self, row:usize) -> Option<Drain<T>> {
        if row < self.rows().into_iter().count() {
            let start = row*self.column_count;
            Some(self.data.drain(start..start+self.column_count))
        }else {
            None
        }
    }
    pub fn insert_row(&mut self, row: impl Iterator<Item = T>) -> Result<(), Drain<T>> {
        let mut inserted_columns = 0;
        self.data.extend(row.inspect(|_| {
            inserted_columns += 1;
        }));
        if self.column_count == inserted_columns {
            Ok(())
        } else {
            let len = self.data.len();
            Err(self.data.drain((len - inserted_columns)..len))
        }
    }
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn into_data(self) -> Vec<T> {
        self.data
    }
    pub fn row(
        &self,
        row:usize
    ) -> Option<&[T]> {
        if row < self.rows().into_iter().count() {
            let start = row*self.column_count;
            self.data.get(start..start+self.column_count)
        }else {
            None
        }
    }
    pub fn rows(
        &self,
    ) -> impl IntoIterator<IntoIter = impl DoubleEndedIterator<Item = &[T]>, Item = &[T]> {
        self.data.chunks(self.column_count)
    }
    pub fn rows_mut(
        &mut self,
    ) -> impl IntoIterator<IntoIter = impl DoubleEndedIterator<Item = &mut [T]>, Item = &mut [T]>
    {
        self.data.chunks_mut(self.column_count)
    }
    pub fn cols(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        let row_count = self.data.len() / self.column_count;
        (0..self.column_count)
            .map(move |c| (0..row_count).map(move |r| &self.data[r * self.column_count + c]))
    }

    pub fn cols_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
        let row_count = self.data.len() / self.column_count;
        let column_count = self.column_count;
        let data_ptr = self.data.as_mut_ptr();
        (0..self.column_count).map(move |c| {
            (0..row_count).map(move |r| {
                // SAFETY: these indices are guaranteed to be unique
                unsafe { data_ptr.add(r * column_count + c).as_mut().unwrap() }
            })
        })
    }

    pub fn indices(&self) -> impl Iterator<Item = ((usize, usize), usize)> {
        let row_count = self.data.len() / self.column_count;
        let column_count = self.column_count;
        (0..column_count)
            .flat_map(move |c| (0..row_count).map(move |r| ((c, r), r * column_count + c)))
    }
    pub fn map_ref<U, F: FnMut(&T) -> U>(&self, f: F) -> Grid<U> {
        Grid {
            data: self.data.iter().map(f).collect(),
            column_count: self.column_count,
        }
    }
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Grid<U> {
        Grid {
            data: self.data.into_iter().map(f).collect(),
            column_count: self.column_count,
        }
    }
    pub fn zip<U>(self, other: Grid<U>) -> Grid<(T, U)> {
        Grid {
            data: self.data.into_iter().zip(other.data).collect(),
            column_count: self.column_count,
        }
    }
    pub fn get(&self, (c, r): (usize, usize)) -> Option<&T> {
        if c < self.column_count {
            self.data.get(r * self.column_count + c)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, (c, r): (usize, usize)) -> Option<&mut T> {
        if c < self.column_count {
            self.data.get_mut(r * self.column_count + c)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (c, r): (usize, usize)) -> &Self::Output {
        &self.data[r * self.column_count + c]
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (c, r): (usize, usize)) -> &mut Self::Output {
        &mut self.data[r * self.column_count + c]
    }
}
