pub struct QuickUnion {
    items: Vec<usize>
}

impl QuickUnion {

    pub fn new(length: usize) -> QuickUnion {
        let mut quick_union = QuickUnion {
            items: vec![0; length]
        };

        for i in 0..length {
            quick_union.items[i] = i;
        }

        quick_union
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let mut m: usize;
        let mut n: usize;

        {
            m = self.get_root(i);
            n = self.get_root(j);
        }

        self.items[n] = m;
    }

    pub fn is_connected(&self, i: usize, j: usize) -> bool {
        self.get_root(i) == self.get_root(j)
    }

    fn get_root(&self, i: usize) -> usize {
        let mut j: usize = i;
        loop {
            if j == self.items[j] {
                break;
            }
            j = self.items[j];
        }
        j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_initializes_items_to_nth() {
        let quick_union = QuickUnion::new(4);
        assert_eq!(0, quick_union.items[0]);
        assert_eq!(1, quick_union.items[1]);
        assert_eq!(2, quick_union.items[2]);
        assert_eq!(3, quick_union.items[3]);
    }

    #[test]
    pub fn union_changes_root_of_first_item_to_second_item_root() {
        let mut quick_union = QuickUnion::new(4);

        quick_union.union(0, 1);
        assert_eq!(0, quick_union.items[0]);
        assert_eq!(0, quick_union.items[1]);
        assert_eq!(2, quick_union.items[2]);
        assert_eq!(3, quick_union.items[3]);

        quick_union.union(2, 3);
        assert_eq!(0, quick_union.items[0]);
        assert_eq!(0, quick_union.items[1]);
        assert_eq!(2, quick_union.items[2]);
        assert_eq!(2, quick_union.items[3]);

        quick_union.union(0, 2);
        assert_eq!(0, quick_union.items[0]);
        assert_eq!(0, quick_union.items[1]);
        assert_eq!(0, quick_union.items[2]);
        assert_eq!(2, quick_union.items[3]);
    }

    #[test]
    fn is_connected_returns_true_when_union_executed() {
        let mut quick_union = QuickUnion::new(4);

        quick_union.union(0, 3);

        assert!(!quick_union.is_connected(0, 1));
        assert!(!quick_union.is_connected(0, 2));
        assert!(quick_union.is_connected(0, 3));

        quick_union.union(1, 2);

        assert!(!quick_union.is_connected(1, 0));
        assert!(!quick_union.is_connected(1, 3));
        assert!(quick_union.is_connected(1, 2));
    }

}