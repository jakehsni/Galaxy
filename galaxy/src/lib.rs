pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod storage;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity() {
        let stor = storage::Storage::<i32>::with_capacity(23);
        assert_eq!(23, stor.capacity());

    }

    #[test]
    fn grow_exact() {
        let mut stor = storage::Storage::<i32>::with_capacity(75);
        assert_eq!(75, stor.capacity());
        stor.grow_exact(100);
        assert_eq!(175, stor.capacity());



    }

    #[test]
    fn grow_mul() {
        let mut stor = storage::Storage::<i32>::with_capacity(7);
        assert_eq!(7, stor.capacity());
        stor.grow_mul(100);
        assert_eq!(700, stor.capacity());



    }
    #[test]
    fn grow() {
        let mut stor = storage::Storage::<i32>::with_capacity(7);
        let mut stor0 = storage::Storage::<i32>::new();

        assert_eq!(7, stor.capacity());
        assert_eq!(0, stor0.capacity());

        stor.grow();
        stor0.grow();
        assert_eq!(14, stor.capacity());
        assert_eq!(1, stor0.capacity());




    }
}
