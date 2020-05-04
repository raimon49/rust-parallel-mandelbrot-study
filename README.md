# rust-parallel-mandelbrot-study

## Build & Run

```bash
$ cargo build --release
$ target/release/mandelbrot /tmp/mandel.png 4000x3000 -1.20,0.35 -1,0.20
```

### Example result

![](mandel.png)

## Testing

```bash
$ cargo test
```

## Environment

* rustc 1.40.0 (73528e339 2019-12-16)
* cargo 1.40.0 (bc8e4c8be 2019-11-22)

