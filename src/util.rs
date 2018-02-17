use std::{
    mem,
    ops,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_update() {
        let mut b = Bounds::from_value(0, 0);
        b.update(1, 1);
        b.update(2, -1);
        assert_eq!(b, Bounds {
            max: 1,
            min: -1,
            max_index: 1,
            min_index: 2,
        });
    }

    #[test]
    fn data_cap() {
        let mut data = Data::with_capacity(3);
        assert_eq!(data.push(2), None);
        assert_eq!(data.push(1), None);
        assert_eq!(data.push(-1), None);
        assert_eq!(data.bounds, Some(Bounds {
            max: 2,
            min: -1,
            max_index: 0,
            min_index: 2,
        }));
        assert_eq!(data.push(0), Some(2));
        assert_eq!(data.data, vec![0, 1, -1]);
        assert_eq!(data.get(0), None);
        assert_eq!(data.get(3), Some(&0));
        assert_eq!(data.bounds, Some(Bounds {
            max: 1,
            min: -1,
            max_index: 1,
            min_index: 2,
        }));
    }
}

/// A struct to keep track of the upper and lower bounds of a dataset.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Bounds<T> {
    max: T,
    min: T,
    max_index: usize,
    min_index: usize,
}

impl<T> Bounds<T> where T: PartialOrd + Copy {
    /// Creates a new `Bounds` with the specified index and value
    pub fn from_value(idx: usize, elem: T) -> Bounds<T> {
        Bounds {
            max: elem,
            min: elem,
            max_index: idx,
            min_index: idx,
        }
    }

    /// Replaces the maximum with the maximum of the iterator. If the iterator is empty, no changes
    /// occur.
    pub fn update_max_iter<'a, I>(&mut self, mut iter: I)
        where
            I: Iterator<Item=(usize, &'a T)>,
            T: 'a,
    {
        iter.next().map(|(idx, &elem)| {
            self.max = elem;
            self.max_index = idx;
            for (idx, &elem) in iter {
                self.update_max(idx, elem);
            }
        });
    }

    /// Replaces the minimum with the minimum of the iterator. If the iterator is empty, no changes
    /// occur.
    pub fn update_min_iter<'a, I>(&mut self, mut iter: I)
        where
            I: Iterator<Item=(usize, &'a T)>,
            T: 'a,
    {
        iter.next().map(|(idx, &elem)| {
            self.min = elem;
            self.min_index = idx;
            for (idx, &elem) in iter {
                self.update_min(idx, elem);
            }
        });
    }

    /// Updates the maximum, and returns whether the maximum was updated
    pub fn update_max(&mut self, idx: usize, elem: T) -> bool {
        if elem >= self.max {
            self.max = elem;
            self.max_index = idx;
            true
        } else {
            false
        }
    }

    /// Updates the minimum, and returns whether the minimum was updated
    pub fn update_min(&mut self, idx: usize, elem: T) -> bool {
        if elem <= self.min {
            self.min = elem;
            self.min_index = idx;
            true
        } else {
            false
        }
    }

    /// Updates the bounds in place
    pub fn update(&mut self, idx: usize, elem: T) {
        self.update_max(idx, elem);
        self.update_min(idx, elem);
    }
}

struct Data<T> {
    data: Vec<T>,
    cap: Option<usize>,
    start: usize,
    len: usize,
    bounds: Option<Bounds<T>>,
}

impl<T> Data<T> {
    /// Creates a new `Data` structure without fixed capacity
    pub fn new() -> Data<T> {
        Data {
            data: Vec::new(),
            cap: None,
            start: 0,
            len: 0,
            bounds: None,
        }
    }

    /// Creates a new `Data` structure with the specified capacity
    pub fn with_capacity(cap: usize) -> Data<T> {
        Data {
            data: Vec::with_capacity(cap),
            cap: Some(cap),
            ..Data::new()
        }
    }

    pub fn iter(&self) -> DataIter<T> {
        DataIter {
            data: &self,
            index: self.start,
        }
    }

    pub fn enumerate(&self) -> DataEnumerate<T> {
        DataEnumerate {
            data: &self,
            index: self.start,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len || idx < self.start {
            None
        } else {
            match self.cap {
                Some(cap) => self.data.get(idx % cap),
                None => self.data.get(idx),
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.start = 0;
        self.bounds = None;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Data<T> where T: PartialOrd + Copy {
    /// Pushes an element, and returns the element (if any) removed to make room
    pub fn push(&mut self, elem: T) -> Option<T> {
        match self.cap {
            Some(cap) => {
                // first check for degenerate case (cap == 0):
                if cap == 0 {
                    return None;
                }
                // check if we need to make room:
                if self.len < cap {
                    // nope, just push
                    self.data.push(elem);
                    self.bounds = Some(self.bounds.map_or(
                            Bounds::from_value(self.len, elem),
                            |mut b| { b.update(self.len, elem); b } ));
                    self.len += 1;
                    None
                } else {
                    // we're removing the first element to make room
                    // we can safely unwrap `bounds` here, because we can only get here if
                    // `self.len >= self.cap > 0`
                    let mut bounds = self.bounds.unwrap();

                    if !bounds.update_max(self.len, elem) && bounds.max_index == self.start {
                        // we just removed the max (and didn't add a new max)
                        // skip first element because it's the one we're removing
                        bounds.update_max_iter(self.enumerate().skip(1))
                    }

                    if !bounds.update_min(self.len, elem) && bounds.min_index == self.start {
                        // we just removed the min (and didn't add a new min)
                        // skip first element because it's the one we're removing
                        bounds.update_min_iter(self.enumerate().skip(1))
                    }

                    self.bounds = Some(bounds);

                    self.start += 1;
                    self.len += 1;

                    Some(mem::replace(&mut self.data[(self.start - 1) % cap], elem))
                }
            },
            None => {
                self.data.push(elem);
                self.len += 1;
                self.bounds = self.bounds.map(|mut b| { b.update(self.len, elem); b });
                None
            },
        }
    }
}

impl<T> ops::Index<usize> for Data<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index)
            .expect(format!("index out of bounds: the len is {} and the start is {} \
                            but the index is {}", self.len(), self.start, index).as_ref())
    }
}

/// An iterator over a `Data` struct
pub struct DataIter<'a, T: 'a> {
    data: &'a Data<T>,
    index: usize,
}

impl<'a, T> Iterator for DataIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let el = self.data.get(self.index);
        self.index += 1;
        el
    }
}

/// An iterator over a `Data` struct produced by the `enumerate` method
pub struct DataEnumerate<'a, T: 'a> {
    data: &'a Data<T>,
    index: usize,
}

impl<'a, T> Iterator for DataEnumerate<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<(usize, &'a T)> {
        let el = self.data.get(self.index);
        self.index += 1;
        el.map(|e| (self.index - 1, e))
    }
}

