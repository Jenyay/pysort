trait QuickSort<T: PartialOrd + Clone> : super::SortAlgorithm<T> {
    fn process(&self, data: &mut Vec<T>, start: usize, end: usize) -> usize;

    fn _sort(&self, data: &mut Vec<T>, start: usize, end: usize) {
        if start < end {
            let pivot_index = self.process(data, start, end);

            if pivot_index > start {
                self._sort(data, start, pivot_index - 1);
            }

            if pivot_index < end {
                self._sort(data, pivot_index + 1, end);
            }
        }
    }

    fn sort(&self, data: &mut Vec<T>) {
        self._sort(data, 0, (data.len() as usize) - 1);
    }
}


// QuickSortEnd
pub struct QuickSortEnd;

impl QuickSortEnd {
    pub fn new() -> Self {
        Self {}
    }
}


impl<T: PartialOrd + Clone> QuickSort<T> for QuickSortEnd {
    fn process(&self, data: &mut Vec<T>, start: usize, end: usize) -> usize {
        let pivot = data[end].clone();
        let mut i = (start as i32) - 1;
        let mut j = end as i32;

        while i < j {
            i += 1;
            j -= 1;

            while data[i as usize] < pivot {
                i += 1;
            }

            while data[j as usize] > pivot && j as usize > start {
                j -= 1;
            }

            if i < j {
                let tmp = data[i as usize].clone();
                data[i as usize] = data[j as usize].clone();
                data[j as usize] = tmp;
            }
        }

        let tmp = data[i as usize].clone();
        data[i as usize] = data[end].clone();
        data[end] = tmp;

        i as usize
    }
}

impl<T: PartialOrd + Clone> super::SortAlgorithm<T> for QuickSortEnd {
    fn sort(&self, data: &mut Vec<T>) {
        QuickSort::sort(self, data);
    }
}

impl super::Named for QuickSortEnd {
    fn get_name(&self) -> &str {
        "Quick sort (pivot - the last element)"
    }
}
