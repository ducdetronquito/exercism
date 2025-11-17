pub fn is_leap_year(year: u64) -> bool {
    year.divisible_by(4) && !year.divisible_by(100) || year.divisible_by(400)
}

trait Divisible {
    fn divisible_by(&self, n: u64) -> bool;
}

impl Divisible for u64 {
    fn divisible_by(&self, n: u64) -> bool {
        self.rem_euclid(n) == 0
    }
}
