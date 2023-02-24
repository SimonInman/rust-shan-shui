use draw::{Drawing, Style, RGB};
use noise::{Perlin, NoiseFn};

// for (var i = 0; i < ptlist.length; i++) {
//     var getCol = function(x, y) {
//       var c = (Noise.noise(x * 0.02, y * 0.02, yoff) * 55 + 200) | 0; <this is a bitwise OR, not a mod.
//       return "rgb(" + c + "," + c + "," + c + ")";
//     };
//     canv += poly(ptlist[i], {
//       fil: getCol(...ptlist[i][ptlist[i].length - 1]),
//       str: "none",
//       wid: 1,
//     });

//     var T = PolyTools.triangulate(ptlist[i], {
//       area: 100,
//       convex: true,
//       optimize: false,
//     });
//     for (var k = 0; k < T.length; k++) {
//       var m = PolyTools.midPt(T[k]);
//       var co = getCol(m[0], m[1]);
//       canv += poly(T[k], { fil: co, str: co, wid: 1 });
//     }
//   }
//   return canv;
// };
pub fn distant_mountain(
    x: f64,
    y: f64,
    seed: f64,
    height: f64,
    length: f64,
    segments: usize,
) -> Vec<Drawing> {
    let span = 10.0;
    let perlin = Perlin::new();
    let debug_rgb = RGB {
        r: 100,
        g: 100,
        b: 100,
    }
    .clone();

    let points = generate_points_list(x, y, 
        height, length, span, segments, perlin, seed);

    let mut out = vec![];

    for inner_list in points {
        let colour = get_colour(*inner_list.last().unwrap(), y, perlin);
        out.push(
            Drawing::new()
                .with_shape(super::line_from(inner_list))
                .with_style(
                    Style::filled(colour)
                ),
        );
    }
    return out;
}

fn get_colour((x, y) : (f64, f64), global_y_offset: f64, perlin: Perlin) -> RGB {
            let this_noise = perlin.get([x * 0.02, y * 0.02, global_y_offset]);
            let rand_colour = (this_noise * 55.0 + 200.0) as u8;
    //         todo continue writing this
    //         RGB()
    let rgb = RGB{r:rand_colour, g:rand_colour, b:rand_colour}.clone();
    return rgb;
}

fn generate_points_list(
    x: f64,
    y: f64,
    height: f64,
    length: f64,
    span: f64,
    segments: usize,
    perlin: Perlin,
    seed: f64,
) -> Vec<Vec<(f64, f64)>> {

    let mut points = vec![];

    for i in 0..((length / (span * segments as f64)) as usize) {
        let this_mountain =
            generate_single_mountain_points(x, y, length, height, span, 
                segments, i, perlin, seed);
        points.push(this_mountain);
    }

    return points;
}

fn generate_single_mountain_points(
    x: f64,
    y: f64,
    length: f64,
    height: f64,
    span: f64,
    segments: usize,
    mountain_index: usize,
    perlin: Perlin,
    seed: f64,
) -> Vec<(f64, f64)> {
    let pi = std::f64::consts::PI;
    let mut mountain_top: Vec<(f64,f64)> = vec![];
    for j in 0..(segments + 1) {
        let global_point_index: f64 = (mountain_index * segments + j) as f64;
        let out = (
            x + global_point_index * span,
            y - height
                * perlin.get([global_point_index * 0.05, seed])
                * (pi * global_point_index * span / length).sin().sqrt(),
        );

        mountain_top.push(out);
    }
    // The JS code uses unshift in the following loop, which adds
    // elements to the front of the list. I think it will be easier to add them to a new
    // list, reverse it and concat the two.
    let mut points_to_reverse_and_prepend = vec![];
    // todo check behaviour when seg is odd
    for j in 0..(segments / 2 + 1) {
        let global_point_index = (mountain_index * segments + j * 2) as f64;
        let out = (
            x + global_point_index * span,
            y + 24.0
                * perlin.get([global_point_index * 0.05, 2.0, seed])
                * (pi * global_point_index * span / length).sin(),
        );
        points_to_reverse_and_prepend.push(out);
    }
    points_to_reverse_and_prepend.reverse();
    points_to_reverse_and_prepend.append(&mut mountain_top);

    return points_to_reverse_and_prepend;
}
