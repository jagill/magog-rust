#[cfg(test)]
mod tests {
    #[test]
    fn check_iter_collect() {
        let orig: Vec<u32> = vec![0, 1, 2];
        let target: Vec<u32> = orig.iter().cloned().collect();
        assert_eq!(orig, target);
    }

    #[test]
    fn check_map() {
        let orig: Vec<u32> = vec![0, 1, 2];
        let target: Vec<u32> = orig.iter().cloned().map(|i| i * 2).collect();
        assert_eq!(vec![0, 2, 4], target);
    }

    #[test]
    fn check_flat_map() {
        let orig: Vec<u32> = vec![1, 2, 3];
        let target: Vec<u32> = orig.into_iter().flat_map(|n| 0..n).collect();
        assert_eq!(vec![0, 0, 1, 0, 1, 2], target);
    }

}
