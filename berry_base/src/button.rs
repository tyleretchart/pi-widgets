extern crate sysfs_gpio;

use self::sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

pub fn poll(pin_num: u64) -> bool {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.

    let input = Pin::new(pin_num);
    let mut pressed: bool = false;
    let mut val: u8 = 1;
    input.with_exported(|| {
        sleep(Duration::from_millis(120));
        input.set_direction(Direction::In)?;
        let mut sum: u8 = 0;
        let mut iters: u8 = 0;
        for _ in 0..20 {
            val = input.get_value()?;
            sum += val;
            iters += 1;

            if iters > 18 {
                if sum == 0 {
                    pressed = true;
                }
                sum = 0;
                iters = 0;
            }
            sleep(Duration::from_millis(10));
        }
        Ok(())
    });
    return pressed;
}
