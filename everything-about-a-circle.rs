use std::f64::consts::PI;

fn main() {
    let mut kind = String::new();
    println!("Do you have the radius, circumference, diameter, or area of the circle?");
    std::io::stdin().read_line(&mut kind).unwrap();
    if kind.starts_with('r') {
        println!("Please input the radius of your circle.");
    } else if kind.starts_with('c') {
        println!("Please input the circumference of your circle.");
    } else if kind.starts_with('d') {
        println!("Please input the diameter of your circle.");
    } else if kind.starts_with('a') {
        println!("Please input the area of your circle.");
    } else {
        println!("Please input radius, circumference, diameter, or area.");
        main();
    }
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let input: f64 = line.trim().parse().unwrap();
    if kind.starts_with('r') {
        println!("{}", Circle::from_radius(input));
    } else if kind.starts_with('c') {
        println!("{}", Circle::from_circumference(input));
    } else if kind.starts_with('d') {
        println!("{}", Circle::from_diameter(input));
    } else if kind.starts_with('a') {
        println!("{}", Circle::from_area(input));
    } else {
        println!("Please input radius, circumference, diameter, or area.");
        main();
    }
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
        Circle::from_radius(diameter / 2.0)
    }

    fn from_circumference(circ: f64) -> Circle {
        Circle::from_radius(circ / PI)
    }

    fn from_radius(radius: f64) -> Circle {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "radius: {}, diameter: {}, area: {}, circumference: {}",
            self.radius, self.diameter, self.area, self.circumference
        )
    }
}
