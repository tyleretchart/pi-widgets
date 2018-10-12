extern crate sysfs_gpio;

use self::sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct LightArguments {
    pub pin: u64,
    pub duration_ms: u64,
    pub period_ms: u64,
}

pub fn blink_led(light_args: LightArguments) {
    match blink_led_raw(light_args) {
        Ok(()) => println!("Success!"),
        Err(err) => println!("We have a blinking problem: {}", err),
    }
}

// Export a GPIO for use.  This will not fail if already exported
fn blink_led_raw(light_args: LightArguments) -> sysfs_gpio::Result<()> {
    let my_led = Pin::new(light_args.pin);
    my_led.with_exported(|| {
        // This extra sleep is needed to make sure that the pin is exported
        sleep(Duration::from_millis(120));
        my_led.set_direction(Direction::Low)?;
        let iterations = light_args.duration_ms / light_args.period_ms / 2;
        for _ in 0..iterations {
            my_led.set_value(0)?;
            sleep(Duration::from_millis(light_args.period_ms));
            my_led.set_value(1)?;
            sleep(Duration::from_millis(light_args.period_ms));
        }
        my_led.set_value(0)?;
        Ok(())
    })
}
