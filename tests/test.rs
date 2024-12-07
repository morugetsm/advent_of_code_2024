#[test]
fn test_sorted() {
    let foo = [3, 2, 2, 1];
    println!("foo.is_sorted() {}", foo.is_sorted());

    let bar: u8 = 33;
    let baz = 30;
    println!("{}.abs_diff({}) {}", bar, baz, bar.abs_diff(baz));
}

#[test]
fn test_cmp() {
    let foo = [1, 5, 3, 3];

    foo.windows(2).for_each(|vals| {
        let [val1, val2] = vals else {
            return;
        };

        match val1.cmp(val2) {
            std::cmp::Ordering::Equal => println!("{val1} == {val2}"),
            std::cmp::Ordering::Greater => println!("{val1} > {val2}"),
            std::cmp::Ordering::Less => println!("{val1} < {val2}"),
        }
    });
}
