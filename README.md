# AHT21B  Driver
I2C driver for the AHT21B temperature and humidity sensor.

## [Examples](examples)
```rust
use aht21b::{Aht21B, relative_humidity, temperature};
use linux_embedded_hal::{Delay, I2cdev};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut driver = Driver::new(dev, Delay);
    if !driver.verify_state() {
        println!("status wrong")
    }

    let ctdata = driver.query().unwrap();
    println!(
        "relative humidity: {:?}\ntemperature: {:?}",
        relative_humidity(ctdata.0),
        temperature(ctdata.1)
    );
}
```