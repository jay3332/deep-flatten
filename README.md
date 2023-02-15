# Deep Flatten

Deep Flatten is a simple utility trait that flattens up to 32 nested Options.


## Example

```rust
use deep_flatten::DeepFlatten;

let x = Some(Some(Some(Some(Some(Some(Some(Some(Some(())))))))));
let flattened = x.deep_flatten();

assert_eq!(flattened, Some(()));
```

Result support coming soon.

<sub>I genuinely don't know why you will ever need this but ok</sub>
