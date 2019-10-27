# nucleo-h743zi rust embedded sample
## build
```bash
cargo build
```

## run
env: openocd, stlink

1. gdb debug
```bash
openocd -f debug/openocd.cfg
cargo run --bin tri_led
```
2. cortex debug
3. generate binary
```bash
cargo objcopy --bin tri_led -- -O binary led.bin
```