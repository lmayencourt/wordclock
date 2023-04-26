# Bug that Rust catch when compiling

## Compute `hour_to_display` and forget to use it! 
````
fn draw_time(&mut self, time: Time) -> Result<()> {
        let mut hour_to_display = time.hour;
        if time.minute >=25 {
            hour_to_display = time.hour+1;
        }
        // 24 -> 12
		if hour_to_display > 12 {
			hour_to_display = hour_to_display - 12 ;
		}
		// midnight
		if hour_to_display == 0 {
			hour_to_display = 12;
		}
        self.set_pixel_from_lut(&BARN_HOURS_LOOKUP_TABLE, (time.hour-1) as usize, RED);
}
````

`````
warning: value assigned to `hour_to_display` is never read
   --> src/display.rs:154:4
    |
154 |             hour_to_display = 12;
    |             ^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` on by default
````
