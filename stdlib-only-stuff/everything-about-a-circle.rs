use std::f64::consts::PI;

fn main() {
    let mut line = String::new();
    println!("Please input the radius of your circle");
    std::io::stdin().read_line(&mut line).unwrap();
    let radius: f64 = line.trim().parse().unwrap();
    println!(
        "diameter: {} units, circumference: {} units, area: {} square units",
        radius * 2.0,
        radius * 2.0 * PI,
        PI * radius.powf(2.0)
    );
}

#[derive(Clone, Copy, Debug)]
struct Circle {
    diameter: f64,
    radius: f64,
    circumference: f64,
    area: f64,
}
impl Circle {
    fn from_area(area: f64) -> Circle {
        Circle::from_radius((area / PI).sqrt())
    }

    fn from_diameter(diameter: f64) -> Circle {
        Circle::from_radius(diameter/2)
    }

    fn from_circumference(circ: f64) -> Circle {
        Circle::from_radius(circ / PI)
    }

    fn from_radius(radius: f64) {
        Circle {
            radius,
            diameter: radius * 2.0,
            circumference: radius * 2.0 * PI,
            area: radius.powf(2.0) * PI,
        }
    }
    fn circumference(&self) -> f64 {
        self.circumference
    }
    fn area(&self) -> f64 {
        self.area
    }
    fn diameter(&self) -> f64 {
        self.diameter
    }
    fn radius(&self) -> f64 {
        self.radius
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "radius: {}, diameter: {}, area: {}, circumference: {}",
            self.radius, self.diameter, self.area, self.circumference
        )
    }
}
