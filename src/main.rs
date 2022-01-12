use draw::*;
use rand::Rng;

mod blob;
mod trees;

fn main() {
    println!("Hello, world!");

// create a canvas to draw on
let mut canvas = Canvas::new(500, 500);

// let line = line_from(vec![(50.0, 10.0), (60.0, 20.0), (40.0, 70.0)]);

let blob = blob::default_blob(50.0, 50.0);

// let line = LineBuilder::new(50.0, 10.0)
// .line_to(50.0, 75.0).build();

// let rgb = RGB{r:222,g:128,b:255}.clone();
// // create a new drawing
// let our_drawing = Drawing::new()
//     // give it a shape
//     .with_shape(blob)
//     // give it a cool style
//     .with_style(Style::filled(rgb));

let tree = trees::default_tree_1(25.0, 150.0);
let forest1 = forest();

// add it to the canvas
// canvas.display_list.add(our_drawing);

for tree_part in forest1 {
    canvas.display_list.add(tree_part);
}

// save the canvas as an svg
render::save(
    &canvas,
    "pictures/forest.svg",
    SvgRenderer::new(),
)
.expect("Failed to save");
}

fn line_from(line_points: Vec<(f64, f64)>) -> Shape {
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

        builder = builder.line_to(point.0 as f32, point.1 as f32);
    }

    return builder.build();
}

fn forest() -> Vec<Drawing>{
    let mut trees = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..10 {
    let mut tree = trees::default_tree_1(
        5.0 + 20.0*i as f64 +  5.0 * rng.gen::<f64>(), 
    150.0 + 10.0 * rng.gen::<f64>());
        trees.append(&mut tree);
    }
    return trees;
}