use embedded_time::{Clock, Timer, rate::Fraction};

extern crate std;
pub struct StdClock {
    start: std::time::Instant,
}

impl Default for StdClock {
    fn default() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}

impl Clock for StdClock {
    type T = u32;
    const SCALING_FACTOR: embedded_time::rate::Fraction = Fraction::new(1, 1);

    fn new_timer<'a, Dur: embedded_time::duration::Duration>(
        &'a self,
        duration: Dur,
    ) -> embedded_time::Timer<
        'a,
        embedded_time::timer::param::OneShot,
        embedded_time::timer::param::Armed,
        Self,
        Dur,
    >
    where
        Dur: embedded_time::fixed_point::FixedPoint,
    {
        Timer::new(&self, duration)
    }

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        let now = std::time::Instant::now();
        let diff = now.duration_since(self.start);
        Ok(embedded_time::Instant::new(diff.as_millis() as u32))
    }
}
