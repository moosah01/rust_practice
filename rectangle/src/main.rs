fn draw_rectangle(width: u32, height: u32) {
    // Calculate perimeter and area
    let perimeter = 2 * (width + height);
    let area = width * height;

    // Convert width and height to strings
    let width_str = width.to_string();
    let height_str = height.to_string();

    // Calculate padding for width and height labels
    let width_padding = width - width_str.len() as u32;
    let height_padding = height - height_str.len() as u32;

    // Print the rectangle
    for i in 0..height {
        for j in 0..width {
            if i == 0 {
                if j == width - 1 {
                    print!(" *");
                } else {
                    print!("*");
                }
            } else if i == height - 1 {
                if j == width - 1 {
                    print!(" *");
                } else {
                    print!("*");
                }
            } else if i == 1 && j > 1 && j < width - 2 {
                print!("-");
            } else if i == height - 2 && j > 1 && j < width - 2 {
                print!("-");
            } else if j == 0 {
                print!("|");
            } else if j == width - 1 {
                print!("|");
            } else if i == height - 1 && j == 1 + width_padding {
                print!(" {} ", height_str);
            } else if j == width - 1 && i == 1 + height_padding {
                print!("{} ", width_str);
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Print the perimeter and area inside the rectangle
    println!("Perimeter: {}", perimeter);
    println!("Area: {}", area);
}

fn main() {
    let width = 20;
    let height = 6;
    draw_rectangle(width, height);
}

