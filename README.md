# Flashing your own code to nrf52811-solum-tag using a raspberry pico as a debug probe

<img src="https://github.com/user-attachments/assets/44798f19-35ec-4036-ac2f-f4b72524710b" width="275" height="206">

### Foreword: 
I recently got a Solum EL029H4WRC price tag (N52811 chip) and decided to flash my own code to it and use the tag as a conference badge.
Since I haven't got a real debug probe, I decided to use a raspberry pi pico for this purpose. 
Below I describe steps for anyone to reproduce my approach, that happens to be the quickest and most affordable solution among the currently existing. 

I'm not going to cover a whole process of how I came to the solution, just simple steps to reproduce the result.
If you are interested in more complex uses including wireles communications via zigbee, please check this https://github.com/OpenEPaperLink/OpenEPaperLink


### STEP 1 - setup the pico-probe
It is a quite straigth forward step: 
- go to https://github.com/raspberrypi/debugprobe/releases/tag/debugprobe-v2.2.1 
- download the uf2 file
- flash it to your raspberry pi pico

Make sure you have probe-rs installed: https://probe.rs/


### STEP 2 - connect the tag to the pico probe, to use the pico as a flasher
- Since the tag has no usb, we need to use debug pins ^_^ I highlighted pins, we will use, in red on an img below.
 ![IMG_9923](https://github.com/user-attachments/assets/6adb3dc1-a590-4757-b081-1e1d1ae6b639)

Pinout:
| TAG  | PICO |
|---|---|
| VCC  | 36 |
| GND  |  3 |
| CLK  |  4 |
| DIO  |  5 |

![IMG_9926](https://github.com/user-attachments/assets/c22cc305-3b42-465e-9f10-ac8be615e70d)

- Since there is only 4 pins, you can simply glue them to get a kinda connector, or solder if you are comfortable with it.


### STEP 3 - flash example code
- clone this repo 
- `cargo build --release`
- connect the pico probe to debug pins of the tag
- `probe-rs download --chip nRF52811_xxAA target/thumbv7em-none-eabi/release/nrf52811-solum-tag`
- keep connected till a code is flashed (like 2 secs)

![IMG_9924](https://github.com/user-attachments/assets/a9bfbc6d-d773-4733-a7a0-96552e507f38)


 
