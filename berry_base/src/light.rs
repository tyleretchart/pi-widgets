extern crate sysfs_gpio;

use self::sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

pub fn blink_led(led: u64, duration_ms: u64, period_ms: u64) {
    match blink_led_once(led, duration_ms, period_ms) {
        Ok(()) => println!("Success!"),
        Err(err) => println!("We have a blinking problem: {}", err),
    }
}

// Export a GPIO for use.  This will not fail if already exported
fn blink_led_once(led: u64, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
    let my_led = Pin::new(led);
    my_led.with_exported(|| {
        // This extra sleep is needed to make sure that the pin is exported
        sleep(Duration::from_millis(120));
        my_led.set_direction(Direction::Low)?;
        let iterations = duration_ms / period_ms / 2;
        for _ in 0..iterations {
            my_led.set_value(0)?;
            sleep(Duration::from_millis(period_ms));
            my_led.set_value(1)?;
            sleep(Duration::from_millis(period_ms));
        }
        my_led.set_value(0)?;
        Ok(())
    })
}
