#[macro_use]
extern crate callbag;

use callbag::operators::{filter, for_each, from_interval, from_iter, map, take};

#[test]
fn test_iter() {
    let v = (0..10).collect::<Vec<usize>>();
    let vx = v.clone();

    pipe!(from_iter(v), for_each(move |x| assert_eq!(x, vx[x])));
}

#[test]
fn test_interval() {
    let v = (0..5).collect::<Vec<usize>>();

    pipe!(
        from_interval(20),
        take(5),
        for_each(move |x| assert_eq!(x, v[x]))
    );
}

#[test]
fn test_map() {
    pipe!(
        from_iter(0..10),
        map(|x| x * 2),
        for_each(|x| assert_eq!(x % 2, 0))
    );
}

#[test]
fn test_filter() {
    pipe!(
        from_iter(vec![1, 2, 3, 4, 5]),
        map(|x| x * 2),
        filter(|x| x % 4 == 0),
        for_each(|x| assert_eq!(x % 4, 0))
    );
}

#[test]
fn test_all() {
    let v = (0..10).collect::<Vec<usize>>();
    let vx = v.clone()
        .iter()
        .map(|x| x * 3)
        .filter(|x| x % 2 == 0)
        .map(|x| format!("The number is {}", x))
        .collect::<Vec<String>>();

    pipe!(
        from_iter(v),
        map(|x| x * 3),
        filter(|x| x % 2 == 0),
        map(|x| format!("The number is {}", x)),
        for_each(move |x| {
            assert_eq!(vx.contains(&x), true);
            println!("{}", x)
        })
    );
}
