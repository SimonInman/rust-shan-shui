use draw::{Drawing, RGB, Style};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::blob::{self, default_f};
// this.tree01 = function(x, y, args) {
//     var args = args != undefined ? args : {};
//     var hei = args.hei != undefined ? args.hei : 50;
//     var wid = args.wid != undefined ? args.wid : 3;
//     var col = args.col != undefined ? args.col : "rgba(100,100,100,0.5)";
//     var noi = args.noi != undefined ? args.noi : 0.5;

pub fn default_tree_1(x: f64, y: f64) -> Vec<Drawing> {
    return tree_1(x, y, 
        50.0,
        3.0,
        0.5,
    );
}

fn tree_1(x: f64, y: f64, height: f64, width: f64, _noise: f64) -> Vec<Drawing> {
  
//     reso = 10;
//     var nslist = [];
//     for (var i = 0; i < reso; i++) {
//       nslist.push([Noise.noise(i * 0.5), Noise.noise(i * 0.5, 0.5)]);
//     }
  

let rgb = RGB{r:100,g:100,b:100}.clone();
let resolution = 10;
let noise_list = generate_2d_noise_list(resolution);

let mut drawings = vec![]; 
let mut line1 = vec![];
let mut line2 = vec![];

(0..resolution).for_each(|i| {
    let nx = x;
    let ny = y - (i as f64 * height/resolution as f64);
    // todo check integer division
    if i >= resolution/4 {
        let mut this_level_leaves = leaves_for_height(nx, ny, width, resolution, i, rgb);
        drawings.append(&mut this_level_leaves);
    }
    line1.push((nx + (noise_list[i].0 - 0.5) * width - width / 2.0, ny));
    line2.push((nx + (noise_list[i].1 - 0.5) * width + width / 2.0, ny));

});

// todo ordering
drawings.push(Drawing::new().with_shape(super::line_from(line1)).with_style(Style::stroked(1, rgb)));
drawings.push(Drawing::new().with_shape(super::line_from(line2)).with_style(Style::stroked(1, rgb)));

return drawings

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

fn leaves_for_height(x: f64, y: f64, width: f64, resolution: usize, leaf_index: usize, colour: RGB) -> Vec<Drawing> {
    let mut leaves = vec![];
    println!("adding leaves for height {}", y);
    for _ in  0..(resolution - leaf_index) / 5 + 1 {
        println!("adding 1 leaves for height {}", y);
        leaves.push(leaf(x, y, width, resolution, leaf_index, colour));
    }
    return leaves;
}

fn leaf(x: f64, y: f64, width: f64, resolution: usize, leaf_index: usize, colour: RGB) -> Drawing {
    let mut rng = rand::thread_rng();
    let pi = std::f64::consts::PI;

    let distance_from_top = resolution as f64 - leaf_index as f64;

    let leaf_x: f64 = x + (rng.gen::<f64>() - 0.5) * width * 1.2 * distance_from_top ;
    let leaf_y = y + (rng.gen::<f64>() - 0.5) * width;
    let length = rng.gen::<f64>() * 20.0 * distance_from_top * 0.2 + 10.0;
    let width = rng.gen::<f64>() * 6.0 + 3.0;
    // todo implement angle
    let angle = ((rng.gen::<f64>() - 0.5) * pi) / 6.0;
    let ret = true;
    let noise = 0.5;

    let leaf_blob = blob::blob(leaf_x, leaf_y, length, width, 0.0, noise, ret, &default_f);


    return Drawing::new().with_shape(leaf_blob).with_style(Style::filled(colour));
}

fn generate_2d_noise_list(len: usize) -> Vec<(f64, f64)> {
    let perlin = Perlin::new();

    let noise_for_index = |i| (
        perlin.get([(i as f64) * 0.05, 0.0]), 
        perlin.get([(i as f64) * 0.05, 0.5]), 
    );
    let out = (0..len).into_iter().map(noise_for_index).collect();

    return out;
}