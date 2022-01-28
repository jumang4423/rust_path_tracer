# rust path tracer

![](his/stage5_6.bmp)

![](his/stage5_7.bmp)

1. setup settings.json
   example:

```json
{
  "width": 640,
  "height": 320,
  "sampling": 64,
  "multi_thread_num": 8,
  "bmp_file_name": "output.bmp",
  "is_gen_ppm": true
}
```

2. run

```
# generates <bmp_file_name>
cargo run --release
```
