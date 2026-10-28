// Rodet: mistakes with named arguments.

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    area(width = 1, depth = 2);
    //~^ ERROR this function has no parameter named `depth`

    area(width = 1, 2);
    //~^ ERROR positional argument after a named argument

    area(3, width = 1);
    //~^ ERROR parameter `width` is given more than once

    area(width = 1);
    //~^ ERROR missing argument for parameter `height`

    area(width = "wide", height = 2);
    //~^ ERROR mismatched types

    let f = |a: u32| a;
    f(a = 1);
    //~^ ERROR named arguments can only be used when calling a function or method directly
}
