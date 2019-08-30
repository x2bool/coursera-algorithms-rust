pub struct CachingUnion {
    items: Vec<usize>
}

impl CachingUnion {

    pub fn new(length: usize) -> CachingUnion {
        let mut weighted_union = CachingUnion {
            items: vec![0; length]
        };

        for i in 0..length {
            weighted_union.items[i] = i;
        }

        weighted_union
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let mut i_root = self.get_root(i);
        let mut j_root = self.get_root(j);

        self.items[j_root] = i_root;
    }

    pub fn is_connected(&mut self, i: usize, j: usize) -> bool {
        self.get_root(i) == self.get_root(j)
    }

    fn get_root(&mut self, i: usize) -> usize {
        let mut j = i;
        loop {
            if self.items[j] == j {
                break;
            }
            j = self.items[j];
        }

        self.set_root(i, j); // save root

        j
    }

    fn set_root(&mut self, i: usize, root: usize) {
        self.items[i] = root;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_initializes_items_to_nth() {
        let caching_union = CachingUnion::new(4);
        assert_eq!(0, caching_union.items[0]);
        assert_eq!(1, caching_union.items[1]);
        assert_eq!(2, caching_union.items[2]);
        assert_eq!(3, caching_union.items[3]);
    }

    #[test]
    pub fn union_changes_root_of_first_item_to_second_item_root() {
        let mut caching_union = CachingUnion::new(4);

        caching_union.union(0, 1);
        assert_eq!(0, caching_union.items[0]);
        assert_eq!(0, caching_union.items[1]);
        assert_eq!(2, caching_union.items[2]);
        assert_eq!(3, caching_union.items[3]);

        caching_union.union(2, 3);
        assert_eq!(0, caching_union.items[0]);
        assert_eq!(0, caching_union.items[1]);
        assert_eq!(2, caching_union.items[2]);
        assert_eq!(2, caching_union.items[3]);

        caching_union.union(0, 2);
        assert_eq!(0, caching_union.items[0]);
        assert_eq!(0, caching_union.items[1]);
        assert_eq!(0, caching_union.items[2]);
        assert_eq!(2, caching_union.items[3]);
    }

    #[test]
    fn is_connected_returns_true_when_union_executed() {
        let mut caching_union = CachingUnion::new(4);

        caching_union.union(0, 3);

        assert!(!caching_union.is_connected(0, 1));
        assert!(!caching_union.is_connected(0, 2));
        assert!(caching_union.is_connected(0, 3));

        caching_union.union(1, 2);

        assert!(!caching_union.is_connected(1, 0));
        assert!(!caching_union.is_connected(1, 3));
        assert!(caching_union.is_connected(1, 2));
    }

}