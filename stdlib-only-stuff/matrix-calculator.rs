fn main() {
    let mut line = String::new();
    println!("Please input matrix values in the form a b c d");
    std::io::stdin().read_line(&mut line).unwrap();
    let items = line.strip_suffix("\n").unwrap().split(" ").collect::<Vec<&str>>();
    let a: i128 = items[0].parse().unwrap();
    let b: i128 = items[1].parse().unwrap();
    let c: i128 = items[2].parse().unwrap();
    let d: i128 = items[3].parse().unwrap();
    println!("{:?}", inverse([[a, b], [c, d]]));
}

const fn inverse(x: [[i128; 2]; 2]) -> [[i128; 2]; 2] {
    let determinate = x[0][0]*x[1][1]-x[0][1]*x[1][0];
    let a = x[0][0] * determinate;
    let b = x[0][1] * determinate;
    let c = x[1][0] * determinate;
    let d = x[1][1] * determinate;
    [[d, -b], [-c, a]]
}
