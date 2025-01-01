struct ColorRegularStruct {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

struct ColorTupleStruct(u32, u32, u32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct {};
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
