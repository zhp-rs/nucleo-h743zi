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
```
## download use stlink command tool
```bash
ST-LINK_CLI.exe -p led.bin 0x08000000 -Rst # Windows
```
## download use openocd
```bash
 openocd -f debug/openocd.cfg   -c init -c halt -c "flash write_image erase led.bin 0x08000000" -c reset -c shutdown
```
## download use pyocd
```bash
 pyocd flash --pack ~/.local/share/Keil.STM32H7xx_DFP.2.3.1.pack -t stm32h743zitx -f 4M -a 0x08000000 led.bin
```
