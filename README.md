# landmine

**Boom.**

```rust
use landmine::Landmine;

fn landmine() {
    let landmine = Landmine::Safe(());
    drop(landmine);

    let number = Landmine::Safe(5);
    let inner = number.unwrap();

    let landmine: Landmine<i32> = Landmine::Armed;
    let defused = landmine.defuse();
}
```