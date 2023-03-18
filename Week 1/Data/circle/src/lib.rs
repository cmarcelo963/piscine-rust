use std::f64::consts::PI;
#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point {
                x,
                y,
            },
            radius,
        }
    }
    pub fn area(&self) -> f64 {
        // let r = ((self.center.Point.x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
        PI * self.radius.powi(2)
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, second_circle: &Circle) -> bool {
        let dx = second_circle.center.x - self.center.x;
        let dy = second_circle.center.y - self.center.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        if distance <= self.radius + second_circle.radius {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn distance(&self, second_point: &Point) -> f64 {
        let dx = second_point.x - self.x;
        let dy = second_point.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	let point_a = Point { x: 1.0, y: 1.0 };
	let point_b = Point { x: 0.0, y: 0.0 };
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);

	println!(
		"distance between {:?} and {:?} is {}",
		point_a,
		point_b,
		point_a.distance(&point_b)
	);
    }
}
