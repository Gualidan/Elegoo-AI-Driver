# Conqueror Robot Tank Pin Map

**Board:** Arduino UNO R3 (ATmega328P, 5 V, 16 MHz)  
**Shield:** ELEGOO Conqueror Car Shield V1.0 (DRV8835) or V1.1 (TB6612)

Pin numbers are the labels printed on the UNO headers. Direction is relative to the UNO.
The two shield revisions use the same pins except for the motor driver, D3, and the
D7/D8 motor-direction pins (see the notes below).

## Quick reference

```text
D0   serial RX             D1   serial TX
D2   key / button          D3   TB6612 STBY (V1.1 only)
D4   RGB LED               D5   motor A PWM (right)
D6   motor B PWM (left)    D7   motor direction (see variants)
D8   motor direction       D9   IR receiver
D10  servo pan             D11  servo tilt
D12  ultrasonic echo       D13  ultrasonic trigger
A0   line tracking right   A1   line tracking middle
A2   line tracking left    A3   battery voltage
A4   I2C SDA               A5   I2C SCL
```

## Arduino UNO pin map

| Pin | Signal | Direction | Notes |
| --- | --- | --- | --- |
| **D0 / RX** | serial RX | input | Receives serial data from the ESP32 camera module (UART header P8, pin 2). Slide switch **S1** selects whether D0 listens to USB serial or the ESP32. |
| **D1 / TX** | serial TX | output | Sends serial data to the ESP32 camera module (UART header P8, pin 1) and to the USB serial monitor. |
| **D2** | key / button | input | Active LOW, `INPUT_PULLUP` plus a 10 kΩ external pull-up. Uses hardware interrupt INT0. |
| **D3** | TB6612 STBY | output | TB6612 (V1.1) shield only. Not connected on the DRV8835 (V1.0) shield. |
| **D4** | RGB LED data | output | WS2812 / NeoPixel data input, 1 LED. |
| **D5** | motor A PWM | output (PWM) | DRV8835: `AIN2/EN`. TB6612: `PWMA`. Motor A is the **right** motor. |
| **D6** | motor B PWM | output (PWM) | DRV8835: `BIN2/EN`. TB6612: `PWMB`. Motor B is the **left** motor. |
| **D7** | motor direction | output | DRV8835 (V1.0): `BIN1/PH` (motor B / left). TB6612 (V1.1): `AIN1` (motor A / right). |
| **D8** | motor direction | output | DRV8835 (V1.0): `AIN1/PH` (motor A / right). TB6612 (V1.1): `BIN1` (motor B / left). |
| **D9** | IR receiver | input | Output of the 38 kHz IR receiver. A green indicator LED is on the same net. |
| **D10** | servo Z (pan) | output (PWM) | 500–2400 µs, 0–180°. Moves the camera/ultrasonic platform left and right. |
| **D11** | servo Y (tilt) | output (PWM) | 500–2400 µs, 0–180°. Moves the camera/ultrasonic platform up and down. |
| **D12** | ultrasonic echo | input | HC-SR04 / SR04 echo pulse. Measured with `pulseIn()`. |
| **D13** | ultrasonic trigger | output | HC-SR04 / SR04 trigger; 10 µs HIGH pulse. Also the UNO's onboard LED. |
| **A0** | line tracking right | input (analog) | ITR20001 right sensor. |
| **A1** | line tracking middle | input (analog) | ITR20001 middle sensor. |
| **A2** | line tracking left | input (analog) | ITR20001 left sensor. |
| **A3** | battery voltage | input (analog) | Battery voltage through a 10 kΩ / 1.5 kΩ divider. |
| **A4** | I2C SDA | I/O | MPU-6050 data. |
| **A5** | I2C SCL | I/O | MPU-6050 clock. |

### Power and control pins

| Pin | Use |
| --- | --- |
| **5V** | +5 V rail for the sensors, servos, RGB LED, MPU-6050, and ESP32 camera. Also UART header P8 pin 4. |
| **3.3V** | Exposed on the UNO header but not used by the shield; the ESP32 camera has its own 3.3 V regulator. |
| **Vin** | Battery input to the shield power module and to the motor driver `VM` pins. |
| **GND** | Common ground. |
| **RESET** | Reset button on the shield. |
| **AREF** | Not used by the shield. |

## Connector pinouts

The shield uses keyed headers. Pin 1 is marked on the PCB silkscreen.

### Ultrasonic sensor — P9 (4-pin)

| Pin | Signal |
| --- | --- |
| 1 | +5V |
| 2 | **D13** (trigger) |
| 3 | **D12** (echo) |
| 4 | GND |

### Servos

| Header | Pin 1 | Pin 2 | Pin 3 |
| --- | --- | --- | --- |
| P10 — servo Z (pan) | GND | +5V | **D10** |
| P1 — servo Y (tilt) | GND | +5V | **D11** |

### Line tracking — P7 (5-pin)

| Pin | Signal |
| --- | --- |
| 1 | GND |
| 2 | +5V |
| 3 | **A2** (left) |
| 4 | **A1** (middle) |
| 5 | **A0** (right) |

> An older revision of the source has a commented-out mapping of `A0 = left`,
> `A1 = middle`, `A2 = right`. The current firmware and the tutorial use
> `A2 = left`, `A1 = middle`, `A0 = right`.

### IR receiver — P2 (3-pin)

| Pin | Signal |
| --- | --- |
| 1 | **D9** (signal) |
| 2 | GND |
| 3 | +5V |

### RGB LED — WS2812

| Pin | Signal |
| --- | --- |
| 1 | +5V |
| 2 | DOUT (unused) |
| 3 | GND |
| 4 | **D4** (DIN) |

### UART / ESP32 camera — P8 (4-pin)

| Pin | Signal |
| --- | --- |
| 1 | **D1** (UNO TX) |
| 2 | **D0** (UNO RX, through switch S1) |
| 3 | GND |
| 4 | +5V |

Switch **S1** (MSK-12D19) selects the source for D0/RX: USB serial or the ESP32 camera.
Set it to the ESP32 position to receive commands from the camera module.

### MPU-6050 — U2

| Module pin | Connects to |
| --- | --- |
| VCC | +5V |
| GND | GND |
| SCL | **A5** |
| SDA | **A4** |
| XDA | not connected |
| XCL | not connected |
| AD0 | not connected (I2C address 0x68) |
| INT | not connected |

### Power module

| Header / signal | Connects to |
| --- | --- |
| `POWER_IN` pin 1 | Battery positive → SS34 diode → **Vin** |
| `POWER_IN` pin 2 | Battery negative → GND |
| Voltage divider | **A3** (R1 = 10 kΩ, R4 = 1.5 kΩ) |

### Motor connectors

Motor A is the **right** motor and motor B is the **left** motor.

| Connector | Driver output |
| --- | --- |
| M1 / M3 | Motor A (right): AO1 / AO2 |
| M2 / M4 | Motor B (left): BO1 / BO2 |

On the DRV8835 shield, the driver `MODE` pin is tied to +5V. The `EN` pins are driven
with PWM on D5/D6, and the `PH` pins on D7/D8 select direction. On the TB6612 shield,
`AIN2` and `BIN2` are generated on-board by inverter U3, so only `AIN1` and `BIN1`
are connected to the UNO.

## ESP32-WROVER camera module

The camera module is a separate ESP32-WROVER board. It is connected to the UNO by a
4-pin cable and talks to the UNO over `Serial2` at 9600 baud.

### ESP32 ↔ UNO serial link

| ESP32 pin | Function | UNO pin |
| --- | --- | --- |
| **GPIO33** | `Serial2` RX | **D1 / TX** (UART P8 pin 1) |
| **GPIO4** | `Serial2` TX | **D0 / RX** (UART P8 pin 2, via S1) |
| 5V | Power | +5V (UART P8 pin 4) |
| GND | Ground | GND (UART P8 pin 3) |

### Camera interface (`CAMERA_MODEL_M5STACK_WIDE`)

| Camera signal | ESP32 GPIO |
| --- | --- |
| XCLK | 27 |
| SIOD (SDA) | 22 |
| SIOC (SCL) | 23 |
| VSYNC | 25 |
| HREF | 26 |
| PCLK | 21 |
| D0 (`Y2`) | 32 |
| D1 (`Y3`) | 35 |
| D2 (`Y4`) | 34 |
| D3 (`Y5`) | 5 |
| D4 (`Y6`) | 39 |
| D5 (`Y7`) | 18 |
| D6 (`Y8`) | 36 |
| D7 (`Y9`) | 19 |
| RESET | 15 |
| PWDN | not used (`-1`) |

The red LED on the ESP32 shield is a 3.3 V power indicator, not a GPIO.

## Notes for the Rust port

- The current `src/main.rs` only uses **D13 = trigger** and **D12 = echo**.
- **D7 / D8 swap between shield revisions.** Check which motor driver is fitted before
  writing the motor code.
- **D3 is only used by the TB6612 shield** (STBY). On the DRV8835 shield it is free.
- The original firmware changes the Timer0 prescaler to divide-by-256 for motor PWM
  (about 244.4 Hz) and compensates `millis()` / `delay()` for the resulting timing drift.
  If Timer0 is reused in Rust, account for this.
- The Arduino `Servo` library uses Timer1, which affects PWM on D9 and D10. The ELEGOO
  code attaches the servos only while moving them and calls `detach()` afterwards.
- **D13 is also the UNO onboard LED**, so the ultrasonic trigger pulses make it flicker.
- **D0/D1 are shared** between the USB serial port and the ESP32 camera. Switch S1 selects
  which device drives the UNO's RX pin.
- The line-tracking inputs are **A2 = left, A1 = middle, A0 = right**.

## Sources

- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/02 Manual & Main Code & APP/02 Main Program/DRV8835/ConquerorCar_DRV8835_20220322/DeviceDriverSet_xxx0.h`
- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/02 Manual & Main Code & APP/02 Main Program/TB6612/ConquerorCar_TB6612_20220323/DeviceDriverSet_xxx0.h`
- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/02 Manual & Main Code & APP/04 Code of Carmer (ESP32)/ESP32-WROVER-Camera/ESP32_CameraServer_AP_20220120/camera_pins.h`
- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/04 Related chip information/ConquerorCar-Shield-V1.0.pdf`
- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/04 Related chip information/ConquerorCar-Shield-V1.1(TB6612).pdf`
- `ELEGOO Conqueror Robot Tank Kit 2022.03.23/03 Tutoial & Code/` (tutorial pin descriptions)
