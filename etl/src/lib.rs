use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::<char, i32>::new(), |mut tree, (k, v)| {
        for c in v {
            tree.insert(c.to_ascii_lowercase(), *k);
        }
        tree
    })
}
