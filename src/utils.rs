pub fn partition_try<I, E, F>(iter: I, mut f: F) -> (Vec<I::Item>, Vec<E>)
where
    I: Iterator,
    F: FnMut(I::Item) -> Result<I::Item, E>,
{
    let mut ok = Vec::new();
    let mut err = Vec::new();
    for item in iter {
        match f(item) {
            Ok(ok_item) => ok.push(ok_item),
            Err(err_item) => err.push(err_item),
        }
    }
    (ok, err)
}
