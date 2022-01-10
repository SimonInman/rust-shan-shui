use draw::*;

mod blob;

fn main() {
    println!("Hello, world!");

// create a canvas to draw on
let mut canvas = Canvas::new(100, 100);

// let line = line_from(vec![(50.0, 10.0), (60.0, 20.0), (40.0, 70.0)]);

let blob = blob::default_blob(50.0, 50.0);

// let line = LineBuilder::new(50.0, 10.0)
// .line_to(50.0, 75.0).build();

let rgb = RGB{r:222,g:128,b:255}.clone();
// create a new drawing
let our_drawing = Drawing::new()
    // give it a shape
    .with_shape(blob)
    // give it a cool style
    .with_style(Style::filled(rgb));

// add it to the canvas
canvas.display_list.add(our_drawing);

// save the canvas as an svg
render::save(
    &canvas,
    "tests/svg/less_spiky3.svg",
    SvgRenderer::new(),
)
.expect("Failed to save");
}

fn line_from(line_points: Vec<(f64, f64)>) -> Shape {
    println!("entered line_from!");

    let maybe_start = line_points.first();
    if maybe_start == None {
        //error;
    }

    let start = maybe_start.unwrap();

    let mut builder = LineBuilder::new(start.0 as f32, start.1 as f32);

    for point in line_points.iter().skip(1) {

        if point.0.is_nan() {
            println!("something is broken with nans!");
            continue;
        }

        if point.1.is_nan() {
            println!("something is broken with nans!");
            continue;
        }

        println!("adding another point: {} {}!", point.0, point.1);
        builder = builder.line_to(point.0 as f32, point.1 as f32);
    }

    return builder.build();
}