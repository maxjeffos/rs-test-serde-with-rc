# rs-test-serde-with-rc

Shows how to use Serde with `Rc<T>`. The interesting thing is how Serde will serialize/deserialize an `Rc<T>`; it does not de-dupe either in serializing or deserializing. However, serializing an in-memory data structure using `Rc<T>` to JSON will at least yield valid non-deduped, non-referenced results. Similarily, deserializing JSON into a data structure which uses `Rc<T>` does not create clones of `Rc<T>` for identical `T`s; it simply duplicates them.

You need to use the `rc` feature flag in the Cargo.toml like this:
```
serde = { version = "1.0", features = ["rc"] }
```

