// Rodet: mistakes with parameter default values.

fn area(width: u32, height: u32 = 1) -> u32 {
    width * height
}

fn not_const(s: String = String::from("x")) -> String {
    //~^ ERROR cannot call non-const associated function
    s
}

trait Shape {
    fn scale(&self, by: f64 = 1.0);
    //~^ ERROR default parameter values are only supported on functions in a module or an inherent `impl`
}

fn main() {
    area();
    //~^ ERROR missing argument for parameter `width`

    area(height = 2);
    //~^ ERROR missing argument for parameter `width`

    fn nested(x: i32 = 1) -> i32 { x }
    //~^ ERROR default parameter values are only supported on functions in a module or an inherent `impl`
}
