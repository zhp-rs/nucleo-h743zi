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
cargo objcopy --release --bin tri_led -- -O binary led.bin
ST-LINK_CLI.exe -p led.bin 0x08000000 -Rst # Windows
```