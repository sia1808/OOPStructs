use std::f32::consts::PI;

struct Polygon {
    length_sides: f32,
    num_sides: u32,
}

trait Poly {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
    fn radius(&self) -> f32;
    fn apothem(&self) -> f32;
}

impl Poly for Polygon {
    fn perimeter(&self) -> f32 {
        self.num_sides as f32 * self.length_sides
    }
    fn area(&self) -> f32 {
        self.perimeter() * 0.5 * self.apothem()
    }
    fn radius(&self) -> f32 {
        self.length_sides / (2.0 * PI / self.num_sides as f32).sin()
    }
    fn apothem(&self) -> f32 {
        self.length_sides / (2.0 * PI / self.num_sides as f32).tan()
    }
}

fn main() {
    let lst_sides = [6, 12, 24, 128, 256, 512, 1024, 2048, 65536];
    let lst_lengths = [2.0,5.0,7.0,10.0];

    for &num_sides in &lst_sides {
        for &length_sides in &lst_lengths {
            let poly = Polygon{num_sides, length_sides};
            let polyarea = poly.area();
            let circumscribed_area = PI * length_sides.powi(2);
            let inscribed_area = PI * poly.apothem().powi(2);
            
            println!("For a polygon with {} sides and radius of {}:", num_sides, length_sides);
            println!(" Area = {:?}", polyarea);
            println!(" Circumscribed circle has an area of {:?}", circumscribed_area);
            println!(" Inscribed circle has an area of {:?}", inscribed_area);
            println!();
        }
    }
}