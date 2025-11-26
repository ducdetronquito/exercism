pub fn raindrops(n: u32) -> String {
    let mut drops: Vec<String> = vec![];

    if n.divisible_by(3) {
        drops.push(String::from("Pling"))
    }

    if n.divisible_by(5) {
        drops.push(String::from("Plang"))
    }

    if n.divisible_by(7) {
        drops.push(String::from("Plong"))
    }

    if drops.is_empty() {
        n.to_string()
    } else {
        drops.join("")
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
