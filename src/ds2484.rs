#![no_std]

use ds2484::Ds2484Builder;

pub async fn init_ds2484_async<'a, I2C, D>(i2c: &'a mut I2C, delay: D) -> Ds2484Builder<'a, I2C, D>
where
    I2C: embedded_hal_async::i2c::I2c + 'a,
    D: embedded_hal_async::delay::DelayUs + 'a,
{
    let mut ds2848 = Ds2848Builder::default()
    .build_async(&mut i2c, delay)
    .await
    .expect("Failed to build DS2484 instance");
}

#[embassy_executor::task]
async fn ds2484_task<'a, I2C, D>(i2c: &'a mut I2C, delay: D)
where
    I2C: embedded_hal_async::i2c::I2c + 'a,
    D: embedded_hal_async::delay::DelayUs + 'a,
{
    let mut ds2484 = init_ds2484_async(i2c, delay).await;


}