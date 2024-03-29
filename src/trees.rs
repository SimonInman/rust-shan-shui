use draw::{Drawing, Fill, Stroke, Style, RGB};
use noise::{NoiseFn, Perlin};
use rand::{prelude::SliceRandom, Rng};

use crate::blob::{self, blob, default_f};
// this.tree01 = function(x, y, args) {
//     var args = args != undefined ? args : {};
//     var hei = args.hei != undefined ? args.hei : 50;
//     var wid = args.wid != undefined ? args.wid : 3;
//     var col = args.col != undefined ? args.col : "rgba(100,100,100,0.5)";
//     var noi = args.noi != undefined ? args.noi : 0.5;

pub fn default_tree_1(x: f64, y: f64) -> Vec<Drawing> {
    return tree_1(x, y, 50.0, 3.0, 0.5);
}

fn tree_1(x: f64, y: f64, height: f64, width: f64, _noise: f64) -> Vec<Drawing> {
    let rgb = RGB {
        r: 100,
        g: 100,
        b: 100,
    }
    .clone();
    let resolution = 10;
    let noise_list = generate_2d_noise_list(resolution);

    let mut drawings = vec![];
    let mut line1 = vec![];
    let mut line2 = vec![];

    (0..resolution).for_each(|i| {
        let nx = x;
        let ny = y - (i as f64 * height / resolution as f64);
        // todo check integer division
        if i >= resolution / 4 {
            let mut this_level_leaves = tree_1_leaves_for_height(nx, ny, width, resolution, i, rgb);
            drawings.append(&mut this_level_leaves);
        }
        line1.push((nx + (noise_list[i].0 - 0.5) * width - width / 2.0, ny));
        line2.push((nx + (noise_list[i].1 - 0.5) * width + width / 2.0, ny));
    });

    // todo ordering
    drawings.push(
        Drawing::new()
            .with_shape(super::line_from(line1))
            .with_style(Style::stroked(1, rgb)),
    );
    drawings.push(
        Drawing::new()
            .with_shape(super::line_from(line2))
            .with_style(Style::stroked(1, rgb)),
    );

    return drawings;
}

//     var leafcol;
//     if (col.includes("rgba(")) {
//       leafcol = col
//         .replace("rgba(", "")
//         .replace(")", "")
//         .split(",");
//     } else {
//       leafcol = ["100", "100", "100", "0.5"];
//     }
//     var canv = "";
//     var line1 = [];
//     var line2 = [];
//     for (var i = 0; i < reso; i++) {
//       var nx = x;
//       var ny = y - (i * hei) / reso;
//       if (i >= reso / 4) {
//         for (var j = 0; j < (reso - i) / 5; j++) {
//           canv += blob(
//             nx + (Math.random() - 0.5) * wid * 1.2 * (reso - i),
//             ny + (Math.random() - 0.5) * wid,
//             {
//               len: Math.random() * 20 * (reso - i) * 0.2 + 10,
//               wid: Math.random() * 6 + 3,
//               ang: ((Math.random() - 0.5) * Math.PI) / 6,
//               col:
//                 "rgba(" +
//                 leafcol[0] +
//                 "," +
//                 leafcol[1] +
//                 "," +
//                 leafcol[2] +
//                 "," +
//                 (Math.random() * 0.2 + parseFloat(leafcol[3])).toFixed(1) +
//                 ")",
//             },
//           );
//         }
//       }
//       line1.push([nx + (nslist[i][0] - 0.5) * wid - wid / 2, ny]);
//       line2.push([nx + (nslist[i][1] - 0.5) * wid + wid / 2, ny]);
//     }
//     canv +=
//       poly(line1, { fil: "none", str: col, wid: 1.5 }) +
//       poly(line2, { fil: "none", str: col, wid: 1.5 });
//     return canv;
//   };,

fn tree_1_leaves_for_height(
    x: f64,
    y: f64,
    width: f64,
    resolution: usize,
    leaf_index: usize,
    colour: RGB,
) -> Vec<Drawing> {
    let mut leaves = vec![];
    for _ in 0..(resolution - leaf_index) / 5 + 1 {
        leaves.push(tree_1_leaf(x, y, width, resolution, leaf_index, colour));
    }
    return leaves;
}

fn tree_1_leaf(
    x: f64,
    y: f64,
    width: f64,
    resolution: usize,
    leaf_index: usize,
    colour: RGB,
) -> Drawing {
    let mut rng = rand::thread_rng();
    let pi = std::f64::consts::PI;

    let distance_from_top = resolution as f64 - leaf_index as f64;

    let leaf_x: f64 = x + (rng.gen::<f64>() - 0.5) * width * 1.2 * distance_from_top;
    let leaf_y = y + (rng.gen::<f64>() - 0.5) * width;
    let length = rng.gen::<f64>() * 20.0 * distance_from_top * 0.2 + 10.0;
    let width = rng.gen::<f64>() * 6.0 + 3.0;
    // todo implement angle
    let angle = ((rng.gen::<f64>() - 0.5) * pi) / 6.0;
    let ret = true;
    let noise = 0.5;

    let leaf_blob = blob::blob(leaf_x, leaf_y, length, width, angle, noise, ret, &default_f);

    return Drawing::new()
        .with_shape(leaf_blob)
        .with_style(Style::filled(colour));
}

fn generate_2d_noise_list(len: usize) -> Vec<(f64, f64)> {
    let perlin = Perlin::new();

    let noise_for_index = |i| {
        (
            perlin.get([(i as f64) * 0.05, 0.0]),
            perlin.get([(i as f64) * 0.05, 0.5]),
        )
    };
    let out = (0..len).into_iter().map(noise_for_index).collect();

    return out;
}

pub fn default_tree_2(x: f64, y: f64) -> Vec<Drawing> {
    let rgb = RGB {
        r: 100,
        g: 100,
        b: 100,
    }
    .clone();
    return tree_2(x, y, 16.0, 8.0, 5, 0.5, rgb);
}

fn tree_2(
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    clu: usize,
    _noise: f64,
    colour: RGB,
) -> Vec<Drawing> {
    let mut rng = rand::thread_rng();
    let pi = std::f64::consts::PI;

    let mut out = vec![];
    let stochastic_width = width * (rng.gen::<f64>() * 0.75 + 0.5);
    let stochastic_length = height * (rng.gen::<f64>() * 0.75 + 0.5);

    for i in 0..clu {
        let leaf_blob = blob(
            x + random_gaussian() * clu as f64 * 4.0,
            y + random_gaussian() * clu as f64 * 4.0,
            stochastic_length,
            stochastic_width,
            pi / 2.0,
            0.0,
            true,
            &tree_2_blob_f,
        );
        out.push(
            Drawing::new()
                .with_shape(leaf_blob)
                .with_style(Style::filled(colour)),
        );
    }
    return out;
}

// Transliterate:
// function wtrand(func) {
//     var x = Math.random();
//     var y = Math.random();
//     if (y < func(x)) {
//       return x;
//     } else {
//       return wtrand(func);
//     }
//   }

//   function randGaussian() {
//     return (
//       wtrand(function(x) {
//         return Math.pow(Math.E, -24 * Math.pow(x - 0.5, 2));
//       }) *
//         2 -
//       1
//     );
//   }
fn random_gaussian() -> f64 {
    let mut rng = rand::thread_rng();

    loop {
        let x: f64 = rng.gen::<f64>();
        let y: f64 = rng.gen();
        let weird_number: f64 = -24.0 * (x - 0.5).powi(2);
        let guassian: f64 = weird_number.exp();
        if y < guassian {
            return x;
        }
    }
}

pub fn tree_2_blob_f(x: f64) -> f64 {
    let pi = std::f64::consts::PI;

    let a_sin_a = |a: f64| -> f64 { a * (a * pi).sin() };
    if x <= 1.0 {
        return (a_sin_a(x)).sqrt();
    } else {
        return -1.0 * (a_sin_a(x - 2.0)).sqrt();
    }
}

// this.tree03 = function(x, y, args) {
//     var args = args != undefined ? args : {};
//     var hei = args.hei != undefined ? args.hei : 50;
//     var wid = args.wid != undefined ? args.wid : 5;
//     var ben =
//       args.ben != undefined
//         ? args.ben
//         : function(x) {
//             return 0;
//           };
//     var col = args.col != undefined ? args.col : "rgba(100,100,100,0.5)";
//     var noi = args.noi != undefined ? args.noi : 0.5;

pub(crate) fn default_tree_3(x: f64, y: f64) -> Vec<Drawing> {
    let ben_f = |x| 0.0;
    let rgb = RGB {
        r: 100,
        g: 100,
        b: 100,
    }
    .clone();
    return tree_3(x, y, 50.0, 5.0, ben_f, 0.5, rgb);
}

/*
var canv = "";
var blobs = "";
var line1 = [];
var line2 = [];
for (var i = 0; i < reso; i++) {
  var nx = x + ben(i / reso) * 100;
  var ny = y - (i * hei) / reso;
  < blobs for height >
  line1.push([
    nx + (((nslist[i][0] - 0.5) * wid - wid / 2) * (reso - i)) / reso,
    ny,
  ]);
  line2.push([
    nx + (((nslist[i][1] - 0.5) * wid + wid / 2) * (reso - i)) / reso,
    ny,
  ]);
}
var lc = line1.concat(line2.reverse());
canv += poly(lc, { fil: "white", str: col, wid: 1.5 });
canv += blobs;
return canv;
};
*/

pub fn tree_3(
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    ben: fn(f64) -> f64,
    noise: f64,
    colour: RGB,
) -> Vec<Drawing> {
    let resolution = 10;

    let rgb = RGB {
        r: 100,
        g: 100,
        b: 100,
    }
    .clone();
    let resolution = 10;
    let noise_list = generate_2d_noise_list(resolution);

    let mut drawings = vec![];
    let mut line1 = vec![];
    let mut line2 = vec![];

    (0..resolution).for_each(|i| {
        let nx = x + ben(i as f64 / resolution as f64) * 100.0;
        let ny = y - (i as f64 * height / resolution as f64);
        // todo check integer division
        if i >= 5 {
            let mut this_level_leaves =
                tree_3_leaves_for_height(nx, ny, width, i as f64, resolution as f64, rgb);
            drawings.append(&mut this_level_leaves);
        }

        let distance_from_top = (resolution - i) as f64 / resolution as f64;
        line1.push((
            nx + ((noise_list[i].0 - 0.5) * width - width / 2.0) * distance_from_top,
            ny,
        ));
        line2.push((
            nx + ((noise_list[i].1 - 0.5) * width + width / 2.0) * distance_from_top,
            ny,
        ));
    });

    line2.reverse();
    line1.append(&mut line2);
    let white = RGB {
        r: 255,
        g: 255,
        b: 255,
    }
    .clone();
    let trunk_style = Style::new(Fill::new(white), Stroke::new(1, rgb));
    // todo ordering
    drawings.push(
        Drawing::new()
            .with_shape(super::line_from(line1))
            .with_style(trunk_style),
    );
    return drawings;
}

// var shape = function(x) {
//     return Math.log(50 * x + 1) / 3.95;
// };
// var ox = Math.random() * wid * 2 * shape((reso - i) / reso);
// blobs += blob(
//   nx + ox * randChoice([-1, 1]),
//   ny + (Math.random() - 0.5) * wid * 2,
//   {
//     len: ox * 2,
//     wid: Math.random() * 6 + 3,
//     ang: ((Math.random() - 0.5) * Math.PI) / 6,
//     col:
//       "rgba(" +
//       leafcol[0] +
//       "," +
//       leafcol[1] +
//       "," +
//       leafcol[2] +
//       "," +
//       (Math.random() * 0.2 + parseFloat(leafcol[3])).toFixed(3) +
//       ")",
//   },
fn tree_3_leaf(
    x: f64,
    y: f64,
    width: f64,
    height_index: f64,
    resolution: f64,
    colour: RGB,
) -> Drawing {
    let pi = std::f64::consts::PI;
    let mut rng = rand::thread_rng();
    let shape_f = |p: f64| (50.0 * p + 1.0).ln() / 3.95;
    let fraction_up_tree = (resolution - height_index) / resolution;
    let ox = rng.gen::<f64>() * width * 2.0 * shape_f(fraction_up_tree);

    let rand_x = x + ox * vec![-1.0, 1.0].choose(&mut rng).unwrap();
    let rand_y = y + (rng.gen::<f64>() - 0.5) * width * 2.0;
    let length = ox * 2.0;
    let rand_width = rng.gen::<f64>() * 6.0 + 3.0;
    let rand_angle = (rng.gen::<f64>() - 0.5) * pi / 6.0;

    let leaf_blob = blob(
        rand_x, rand_y, length, rand_width, rand_angle, 0.5, true, &default_f,
    );
    return Drawing::new()
        .with_shape(leaf_blob)
        .with_style(Style::filled(colour));
}

// if (i >= reso / 5) {
//     for (var j = 0; j < (reso - i) * 2; j++) {
//         <leaf>
//     }
//   }
fn tree_3_leaves_for_height(
    x: f64,
    y: f64,
    width: f64,
    height_index: f64,
    resolution: f64,
    colour: RGB,
) -> Vec<Drawing> {
    let mut leaves = vec![];
    for _ in 0..(resolution - height_index) as usize * 2 {
        leaves.push(tree_3_leaf(x, y, width, height_index, resolution, colour));
    }
    return leaves;
}
