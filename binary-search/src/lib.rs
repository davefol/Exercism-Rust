pub fn find<T: std::cmp::PartialEq + std::cmp::PartialOrd>(array: &[T], key: T) -> Option<usize> {
    if array.is_empty() {
        return None
    }
    let middle = array.len() / 2;
    match &array[middle] {
        val if val == &key => Some(middle),
        val if val > &key => find(array.split_at(middle).0, key),
        val if val < &key => find(array.split_at(middle + 1).1, key).map(|x| middle + x + 1),
        _ => unreachable!()
    }
}
