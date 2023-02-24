use core::panic;

use draw::*;
use rand::Rng;

mod blob;
mod distant_mountain;
mod trees;
mod texture;

fn main() {
    println!("Hello, world!");

    // create a canvas to draw on
    let mut canvas = Canvas::new(500, 300);

    // let line = line_from(vec![(50.0, 10.0), (60.0, 20.0), (40.0, 70.0)]);

    // let blob = blob::blob(50.0, 50.0, 40.0, 10.0,
    //        0.0,
    //         0.0,
    //         true,
    //         &blob::default_f);

    // let line = LineBuilder::new(50.0, 10.0)
    // .line_to(50.0, 75.0).build();

    let rgb = RGB {
        r: 222,
        g: 128,
        b: 255,
    }
    .clone();
    let style = Style {
        fill: Some(Fill::new(rgb)),
        stroke: Some(Stroke::new(5, rgb)),
    };
    // // create a new drawing
    // let our_drawing = Drawing::new()
    //     // give it a shape
    //     .with_shape(blob)
    //     // give it a cool style
    //     .with_style(style);
    // add it to the canvas
    // canvas.display_list.add(our_drawing);

    // let tree = trees::default_tree_1(25.0, 150.0);
    let forest1 = forest();

    // for tree_part in forest1 {
    //     canvas.display_list.add(tree_part);
    // }

    let test_mountain = crate::distant_mountain::distant_mountain (
        0.0, 200.0,
         0.0, 40.0, 400.0, 5);
    for mountain_part in test_mountain {
        canvas.display_list.add(mountain_part);
    }

    // save the canvas as an svg
    render::save(
        &canvas,
        "pictures/distant_mountains_debug_3.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}

fn line_from(line_points: Vec<(f64, f64)>) -> Shape {
    let maybe_start = line_points.first();
    if maybe_start == None {
        //error;
        panic!();
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

fn forest() -> Vec<Drawing> {
    let mut trees = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let mut tree = trees::default_tree_3(
            5.0 + 20.0 * i as f64 + 5.0 * rng.gen::<f64>(),
            150.0 + 10.0 * rng.gen::<f64>(),
        );
        trees.append(&mut tree);
    }
    return trees;
}
