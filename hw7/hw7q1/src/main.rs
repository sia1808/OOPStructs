// Sia Sharma
// Collaborators: Om Italiya, Nikita Salkar

enum Shape {
    Triangle(f64, f64, f64),
    Rectangle(f64, f64),
    Circle(f64),
}


// Auxilary Function
impl Shape {
    fn new_shape(sides: &[f64]) -> Option<Shape> { // creating new shape 
        match sides.len() { // matching sides based on lengths of 1 2 or 3 for each shape
            3 => Some(Shape::Triangle(sides[0], sides[1], sides[2])),
            2 => Some(Shape::Rectangle(sides[0], sides[1])),
            1 => Some(Shape::Circle(sides[0])),
            _ => None
        }
    }

// Area Function    
    fn area(&self) -> f64 {
        match * self {
            Shape::Triangle(a,b,c) => {
            // Used Heron's Formula
            let semiper = 0.5 * (a + b + c) as f64;
            let val = (semiper) * (semiper - a) * (semiper - b)* (semiper - c) as f64;
            let val2 = f64::powf(val, 0.5);
            val2
            }
            Shape::Rectangle(a,b) => a * b,
            Shape::Circle(a) => std::f64::consts::PI * a * a, 
        }
    }

// Perimeter Function
    fn perimeter(&self) -> f64 {
        match * self {
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Rectangle(a, b) => 2.0 * (a + b),
            Shape::Circle(a) => 2.0 * std::f64::consts::PI * a,
        }
    }

// Doubling the Perimeter Function
    fn double_peri(&self) -> Shape {
        match self {
            Shape::Triangle(a, b, c) => Shape::Triangle(2.0 * a, 2.0 * b, 2.0 * c),
            Shape::Rectangle(a, b) => Shape::Rectangle(2.0 * a, 2.0 * b),
            Shape::Circle(a) => Shape::Circle(2.0 * a),
        }
    }

    // Checking Function
    fn validation(&self) -> bool {
        match * self {
            Shape::Triangle(a, b, c) => a > 0.0 && b > 0.0 && c > 0.0 && a + b > c && a + c > b && b + c > a,
            Shape::Rectangle(a,b) => {
                if a > 0.0 && b > 0.0 {
                    return true;
                } else {
                    return false;
                }
            }
            Shape::Circle(a) => {
                if a > 0.0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

// Verifying methods work for all shape types
pub fn main() {
    let mut tri = Shape::new_shape(&[4.0,5.0,7.0]).unwrap();
    let mut rec = Shape::new_shape(&[5.0, 6.0]).unwrap();
    let mut cir = Shape::new_shape(&[5.0]).unwrap();
    println!(
        "triangle's area = {}, perimeter = {}, double perimeter = {}, validation = {}",
        tri.area(),
        tri.perimeter(),
        tri.double_peri().perimeter(), 
        tri.validation()
    );
    println!(
        "rectangle's area = {}, perimeter = {}, double perimeter = {}, validation = {}",
        rec.area(),
        rec.perimeter(),
        rec.double_peri().perimeter(), 
        rec.validation()
    );
    println!(
        "circle's area = {:?}, perimeter = {}, double perimeter = {}, validation = {}",
        cir.area(),
        cir.perimeter(),
        cir.double_peri().perimeter(), 
        cir.validation()
    );
}