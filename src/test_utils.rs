pub fn unordered_elements_are<T, I>(i: I, v: Vec<T>) -> bool
where
    T: Eq + Clone,
    I: IntoIterator<Item = T>,
{
    let mut v = v.clone();
    for x in i {
        if let Some(pos) = v.iter().position(|y| *y == x) {
            v.remove(pos);
        } else {
            return false;
        }
    }
    v.is_empty()
}

pub fn is_sorted<I>(i: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut i = i.into_iter();
    if let Some(mut prev) = i.next() {
        for next in i {
            if prev > next {
                return false;
            }
            prev = next;
        }
    }
    true
}
