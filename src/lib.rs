pub mod benchmarking {
    use std::time::{Duration, Instant};

    /// Measure a function's execution time.
    /// Measurement is done only once.
    #[inline]
    pub fn bench_once<F, T>(f: F) -> Duration
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now();
        f();
        instant.elapsed()
    }

    /// Measure a function's execution time.
    /// Measurement is executed 'iterations' times.
    #[inline]
    pub fn bench_times<F, T>(iterations: u32, mut f: F) -> Option<Duration>
    where
        F: FnMut() -> T,
    {
        let cap = iterations.try_into().unwrap_or(0);
        let mut vec = Vec::with_capacity(cap);
        for _ in 0..iterations {
            let elapsed_time = bench_once(&mut f);
            vec.push(elapsed_time);
        }
        vec.into_iter().min()
    }

    /// Calculates how many iterations of benching is allowed in order to fit
    /// into the specified time limits.
    #[inline]
    pub fn calc_iterations(one_measurment_takes: Duration, desired_time: Duration) -> u32 {
        let mut div = 1;
        while desired_time / div > one_measurment_takes {
            div *= 10
        }
        // For some reason, testing with unmodified answer takes 2x from..
        // ..expected time, that's why there is a correction here.
        div / 2
    }
}
