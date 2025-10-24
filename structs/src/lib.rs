pub fn add( first: u64, second: u64 ) -> u64 {
    return first+second;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name : &str) -> String {
    return format!("Hello {name}!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_fn() {
        let result;
        result = add(2, 4);
        assert_eq!(result,6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
                let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

#[cfg(test)]
mod third_test {
    use super::*;

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Ayushmaan");
        assert!(result.contains("Ayushmaan"), "Name was not found in the func, result was `{result}`");

    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod fourth_test {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}