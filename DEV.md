# Building Blocks

## WatchEvent

```rust
let mut stream = pods.watch(&WatchParams::default(), "0").await?.boxed();
while let Some(event) = stream.try_next().await? {
    match event {
        WatchEvent::Added(pod) => info!("Added: {}", pod.name_any()),
        WatchEvent::Modified(pod) => info!("Modified: {}", pod.name_any()),
        WatchEvent::Deleted(pod) => info!("Deleted: {}", pod.name_any()),
        WatchEvent::Error(e) => info!("Error: {}", e.message),
        _ => {}
    };
}
```
