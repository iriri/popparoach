```rust
fn lol() {
   let xs = [0, 1, 2, 3];
   assert_eq!(xs.cut_at::<2>(), (xs.cut::<0, 2>(), xs.cut::<2, 4>()));
}
```
