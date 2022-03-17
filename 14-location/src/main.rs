use std::fmt::Display;

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        println!("{}", self);
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Unknown => write!(f, "unknown location"),
            Location::Anonymous => write!(f, "anonymous location"),
            Location::Known(x,y) => write!(f, "({},{})", x, y),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
