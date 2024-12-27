# How to start server?

```
cargo watch -q -c -w src/ -x run
```

# How to start client side?

```
cargo watch -q -c -w tests/ -x 'test -q quick_dev -- --nocapture'
```
