# callbag.rs 👜

Experimental [callbag](https://github.com/callbag/callbag) implementation in Rust 🦀! Currently attempting to implement some of the core basics (see the list below for progress).

```rust
pipe!(
    from_iter(0..10),
    map(|x| x * 3),
    filter(|x| x % 2 == 0),
    map(|x| format!("The number is {}", x)),
    for_each(|x| println!("{}", x))
);
```

## Testing

Run tests with `cargo test -- --nocapture` to see output.

## API

The list below shows what's currently implemented/being attempted.

### Source factories

- [x] `from_iter`
- [x] `interval`

### Sink factories

- [x] `for_each`

### Transformation operators

- [x] `map`
- [x] `scan`
- [x] `flatten`

### Filtering operators

- [x] `take`
- [ ] `take_until`
- [ ] `take_while`
- [x] `take_until_interval`
- [x] `skip`
- [ ] `skip_until`
- [ ] `skip_while`
- [x] `filter`

### Combination operators

- [x] `merge` - variadic with macro `merge!`
- [ ] `concat`
- [ ] `combine` - tough one, need to flatten tuples

### Utilities

- [x] `pipe` - variadic with macro `pipe!`


