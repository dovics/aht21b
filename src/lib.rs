mod crc;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Operation as I2cOperation, Transactional};

const ADDR: u8 = 0x38;

pub struct Aht21B<I2C, DELAY> {
    i2c: I2C,
    delay: DELAY,
}

impl<I2C, DELAY> Aht21B<I2C, DELAY>
where
    I2C: Transactional,
    DELAY: DelayMs<u8>,
{
    pub fn new(i2c: I2C, delay: DELAY) -> Self {
        Aht21B { i2c, delay }
    }

    pub fn verify_state(&mut self) -> bool {
        let mut read_buffer = [0];
        let mut ops = [
            I2cOperation::Write(&[0x71]),
            I2cOperation::Read(&mut read_buffer),
        ];

        match self.i2c.exec(ADDR, &mut ops) {
            Err(_err) => false,
            Ok(_) => read_buffer[0] & 0x18 == 0x18,
        }
    }

    pub fn query(&mut self) -> Result<(u32, u32), I2C::Error> {
        let mut read_buffer = [0; 7];
        let mut ops1 = [I2cOperation::Write(&[0xAC, 0x33, 0x00])];
        self.i2c.exec(ADDR, &mut ops1)?;
        self.delay.delay_ms(80);

        let mut ops2 = [I2cOperation::Read(&mut read_buffer)];
        self.i2c.exec(ADDR, &mut ops2)?;
        if crc::check_crc8(&read_buffer[0..7].to_vec()) == read_buffer[6] {
            panic!("crc error")
        }

        let mut ctdata = (0, 0);
        for i in 0..3 {
            ctdata.0 = (ctdata.0 << 8) | read_buffer[i] as u32;
            ctdata.1 = (ctdata.1 << 8) | read_buffer[3 + i] as u32;
        }
        ctdata.0 >>= 4;
        ctdata.1 &= 0xfffff;

        Ok(ctdata)
    }
}

pub fn relative_humidity(signal: u32) -> f64 {
    signal as f64 / 1024_f64 / 1024_f64 * 100_f64
}

pub fn temperature(signal: u32) -> f64 {
    signal as f64 / 1024_f64 / 1024_f64 * 200_f64 - 50_f64
}
