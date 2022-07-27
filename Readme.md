# Aligned Vec

Small set of functions to create and optionally initialise and page-lock
`Vec` objects.

## Example

```rust
    fn page_alignedi_test() {
        let ps = page_size::get();
        let len = 5 * ps;
        let capacity = 2 * len;
        let init_value = 42;
        let v = page_aligned_vec::<u8>(len, capacity, Some(init_value), false);
        assert_eq!(v.as_ptr() as usize % ps, 0);
        assert_eq!(v.len(), len);
        assert_eq!(v.capacity(), capacity);
        assert_eq!(v[ps], init_value);
    }
```
