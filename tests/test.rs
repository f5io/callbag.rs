use callbag::{
    merge,
    operators::{filter, flatten, for_each, from_iter, interval, map, merge, scan, skip, take},
    pipe,
};

#[test]
fn test_iter() {
    let v = (0..10).collect::<Vec<usize>>();
    let vx = v.clone();

    pipe!(from_iter(v), for_each(move |x| assert_eq!(x, vx[x])));
}

#[test]
fn test_scan() {
    pipe!(
        from_iter(1..25),
        scan(|a, b| a + b, 0),
        for_each(|x| println!("test_scan: {}", x))
    )
}

#[test]
fn test_interval() {
    let v = (0..5).collect::<Vec<usize>>();

    pipe!(
        interval(20),
        take(5),
        for_each(move |x| {
            println!("test_interval: {}", x);
            assert_eq!(x, v[x])
        })
    );
}

#[test]
fn test_flatten() {
    let v = vec![11, 21, 31, 12, 22, 32, 13, 23, 33];

    pipe!(
        from_iter(1..4),
        map(|x| { pipe!(from_iter(vec![10, 20, 30]), map(move |y| x + y)) }),
        flatten,
        for_each(move |x| {
            println!("test_flatten: {}", x);
            assert_eq!(v.contains(&x), true);
        })
    )
}

#[test]
fn test_merge() {
    pipe!(
        merge(from_iter(0..10), pipe!(interval(10), take(20))),
        take(20),
        for_each(|x| { println!("test_merge: {}", x) })
    )
}

#[test]
fn test_merge_macro() {
    pipe!(
        merge!(from_iter(100..110), interval(10), interval(9)),
        take(30),
        for_each(|x| { println!("test_merge_macro: {}", x) })
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
fn test_skip() {
    let v = (3..10).collect::<Vec<usize>>();

    pipe!(
        from_iter(1..10),
        skip(2),
        for_each(move |x| {
            println!("test_skip: {}", x);
            assert_eq!(v.contains(&x), true);
        })
    );
}

#[test]
fn test_all() {
    let v = (0..10).collect::<Vec<usize>>();
    let vx = v
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
