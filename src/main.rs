fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining: {remaining}");
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.0 * r) as i32;
            let ig = (255.0 * g) as i32;
            let ib = (255.0 * b) as i32;

            println!("{ir} {ig} {ib}")
        }
    }
    eprintln!("Done")
}