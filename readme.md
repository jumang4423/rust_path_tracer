# rust ray tracer

1. setup settings.json
example:
```json
{
    "width": 640,
    "height": 320,
    "sampling": 64,
    "bmp_file_name": "output.bmp",
    "is_gen_ppm": true
}
```

2. run
```
cargo run --release && open *.ppm
```