
pub struct QuickFind {
    items: Vec<i32>
}

impl QuickFind {

    pub fn new(length: usize) -> QuickFind {
        let mut finder = QuickFind {
            items: vec![0; length]
        };

        for i in 0..length {
            finder.items[i] = i as i32;
        }

        finder
    }

    pub fn union(&mut self, i: usize, j: usize) {
        if i >= self.items.len() || j >= self.items.len() {
            return;
        }

        let m = self.items[i];
        let n = self.items[j];

        if n == m {
            return;
        }

        for k in 0..self.items.len() {
            if self.items[k] == n {
                self.items[k] = m;
            }
        }
    }

    pub fn is_connected(&self, i: usize, j: usize) -> bool {
        if i >= self.items.len() || j >= self.items.len() {
            return false;
        }

        self.items[i] == self.items[j]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_finder_has_length_of_n() {
        let n: usize = 256;
        let finder = QuickFind::new(n);
        assert_eq!(n, finder.items.len());
    }

    #[test]
    fn new_finder_has_items_initialized_to_nth() {
        let finder = QuickFind::new(4);
        assert_eq!(0, finder.items[0]);
        assert_eq!(1, finder.items[1]);
        assert_eq!(2, finder.items[2]);
        assert_eq!(3, finder.items[3]);
    }

    #[test]
    fn union_changes_value_of_j_to_value_of_i() {
        let mut finder = QuickFind::new(4);

        finder.union(0, 3);

        assert_eq!(0, finder.items[0]);
        assert_eq!(0, finder.items[3]);

        finder.union(1, 2);

        assert_eq!(1, finder.items[1]);
        assert_eq!(1, finder.items[1]);

        finder.union(0, 1);

        for i in 0..finder.items.len() {
            assert_eq!(0, finder.items[i]);
        }
    }

    #[test]
    fn is_connected_returns_true_when_union_executed() {
        let mut finder = QuickFind::new(4);

        finder.union(0, 3);

        assert!(!finder.is_connected(0, 1));
        assert!(!finder.is_connected(0, 2));
        assert!(finder.is_connected(0, 3));

        finder.union(1, 2);

        assert!(!finder.is_connected(1, 0));
        assert!(!finder.is_connected(1, 3));
        assert!(finder.is_connected(1, 2));
    }

}