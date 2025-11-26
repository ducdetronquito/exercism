pub fn raindrops(n: u32) -> String {
    let mut drops = String::new();

    if n.divisible_by(3) {
        drops += "Pling";
    }

    if n.divisible_by(5) {
        drops += "Plang";
    }

    if n.divisible_by(7) {
        drops += "Plong";
    }

    if drops.is_empty() {
        n.to_string()
    } else {
        drops
    }
}

pub trait Divisible {
    fn divisible_by(self, n: u32) -> bool;
}

impl Divisible for u32 {
    fn divisible_by(self, n: u32) -> bool {
        self.rem_euclid(n) == 0
    }
}
