#[macro_use]
extern crate callbag;

use callbag::operators::{combine, filter, for_each, from_interval, from_iter, map, take};
use std::{thread, time};

#[test]
fn keep_thread_open() {
    thread::sleep(time::Duration::from_millis(500));
}

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
        for_each(move |x| {
            println!("test_interval: {}", x);
            assert_eq!(x, v[x])
        })
    );
}

#[test]
fn test_combine() {
    pipe!(
        combine(
            from_iter(0..10),
            pipe!(from_interval(10), take(20))
        ),
        take(20),
        for_each(|x| {
            println!("test_combine: {}", x)
        })
    )
}

#[test]
fn test_combine_macro() {
    pipe!(
        combine!(
            from_iter(100..110),
            from_interval(10),
            from_interval(9)
        ),
        take(30),
        for_each(|x| {
            println!("test_combine_macro: {}", x)
        })
    )
}

#[test]
fn test_map() {
    pipe!(
        from_iter(0..10),
        map(|x| x * 2),
        for_each(|x| {
            println!("test_map: {}", x);
            assert_eq!(x % 2, 0)
        })
    );
}

#[test]
fn test_filter() {
    pipe!(
        from_iter(vec![1, 2, 3, 4, 5]),
        map(|x| x * 2),
        filter(|x| x % 4 == 0),
        for_each(|x| {
            println!("test_filter: {}", x);
            assert_eq!(x % 4, 0)
        })
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
