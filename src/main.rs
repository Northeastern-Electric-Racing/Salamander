#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::{bind_interrupts, dma, peripherals};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod ds2484;

bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_CHANNEL1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>;
});

