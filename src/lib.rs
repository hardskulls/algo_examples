pub mod benchmarking
{
    use std::time::{Duration, Instant};

    /// Measures the execution time of a function once and returns the duration.
    #[inline]
    pub fn bench_once<F, T>(f : F) -> Duration
        where
            F : FnOnce() -> T,
    {
        let instant = Instant::now();
        f();
        instant.elapsed()
    }

    /// Measure a function's execution time.
    ///
    /// 'iterations' defines how many measurements there will be.
    /// Returns the lowest value, thereby minimizing OS influence on results.
    #[inline]
    pub fn bench_times<F, T>(iterations : u32, mut f : F) -> Option<Duration>
        where
            F : FnMut() -> T,
    {
        let cap = iterations.try_into().unwrap_or(0);
        let mut vec = Vec::with_capacity(cap);
        for _ in 0..iterations
        {
            let elapsed_time = bench_once(&mut f);
            vec.push(elapsed_time);
        }
        vec.into_iter().min()
    }

    /// Calculates the number of iterations of `bench_once` that can be executed within a specified time limit.
    #[inline]
    pub fn calc_iterations(one_measurement_takes : Duration, desired_time : Duration) -> u32
    {
        let mut div = 1;
        while desired_time / div > one_measurement_takes
        { div *= 10 }
        // For some reason, testing with an unmodified answer takes twice the expected time;
        // that's why there is a correction here.
        div / 2
    }
}

pub mod shorthands
{
    use std::{collections::HashMap, hash::Hash};

    pub fn at_idx(idx : usize) -> std::ops::RangeInclusive<usize>
    { idx ..= idx }

    pub fn new_h_map<'a, K, const L : usize>(arr : [(&'a K, i32); L])
        -> HashMap<&'a K, i32>
        where
            K : Eq + Hash + ?Sized,
    { HashMap::from(arr) }
}

pub mod formatting
{
    pub fn is_emoji(c : char) -> bool
    {
        match c
        {
            '\u{01F600}' ..= '\u{01F64F}' |
            '\u{01F300}' ..= '\u{01F5FF}' |
            '\u{01F680}' ..= '\u{01F6FF}' |
            '\u{01F1E0}' ..= '\u{01F1FF}' |
            '\u{002702}' ..= '\u{0027B0}' |
            '\u{0024C2}' ..= '\u{01F251}' => true,
            _ => false,
        }
    }

    pub fn on_screen_len(s : &str) -> usize
    {
        let count = |acc, c| acc + if is_emoji(c) { 2 } else { 1 };
        s.chars().fold(0, count)
    }
}

#[cfg(test)]
mod tests
{
    use crate::formatting::{is_emoji, on_screen_len};

    #[test]
    fn is_emoji_test()
    {
        "✨✅🚧❌"
            .chars()
            .for_each(|c| assert_eq!(is_emoji(c), true));
    }

    #[test]
    fn emojis_len_and_count_test()
    {
        assert_eq!(on_screen_len("✨✅🚧❌"), 8);
        assert_eq!(on_screen_len("✨ ✅🚧❌"), 9);
        assert_eq!(on_screen_len("✨ ✅ 🚧❌"), 10);
        assert_eq!(on_screen_len("✨ ✅ 🚧 ❌"), 11);
        assert_eq!(on_screen_len("✨ ✅ 🚧 ❌ "), 12);

        let s = "✨ It works! Answer is 6 ✅";
        assert_eq!(on_screen_len(s), 27);

        let s = "🚧 Oh, shieeet, answer is 6 instead of 5 ❌";
        assert_eq!(on_screen_len(s), 43);
    }
}
