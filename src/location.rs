#[derive(Debug, Clone)]
pub struct Location {
    x: f32,
    y: f32,
}

impl Location {
    pub fn origin() -> Location {
        Location { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Location {
        Location { x, y }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Location {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn translate(&self, other: &Self) -> Self {
        Location::new(self.x + other.x, self.y + other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing_true_equality() {
        let l1 = Location::new(1.0, 1.0);
        let l2 = Location::new(1.0, 1.0);
        let bool = l1 == l2;
        assert_eq!(bool, true);
    }

    #[test]
    fn testing_false_equality() {
        let l1 = Location::new(1.0, 1.0);
        let l2 = Location::new(2.0, 1.0);
        let bool = l1 == l2;
        assert_eq!(bool, false);
    }

    #[test]
    fn testing_translation_positive() {
        let l1 = Location::new(2.0, 3.0);
        let l2 = Location::new(4.0, 5.0);
        let l3 = l1.translate(&l2);
        assert_eq!(l3, Location::new(6.0, 8.0));
    }

    #[test]
    fn testing_translation_negative() {
        let l1 = Location::new(2.0, 3.0);
        let l2 = Location::new(-4.0, 5.0);
        let l3 = l1.translate(&l2);
        assert_eq!(l3, Location::new(-2.0, 8.0));
    }

    #[test]
    fn testing_translation_identity() {
        let l1 = Location::new(2.0, 3.0);
        let l2 = Location::origin();
        let l3 = l1.translate(&l2);
        assert_eq!(l3, l1);
    }
}
