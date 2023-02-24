use std::default;

use draw::{Drawing, Style, RGB};
use rand::Rng;

//   var texture = function(ptlist, args) {
//     var args = args != undefined ? args : {};
//     var xof = args.xof != undefined ? args.xof : 0;
//     var yof = args.yof != undefined ? args.yof : 0;
//     var tex = args.tex != undefined ? args.tex : 400;
//     var wid = args.wid != undefined ? args.wid : 1.5;
//     var len = args.len != undefined ? args.len : 0.2;
//     var sha = args.sha != undefined ? args.sha : 0;
//     var ret = args.ret != undefined ? args.ret : 0;
//     var noi =
//       args.noi != undefined
//         ? args.noi
//         : function(x) {
//             return 30 / x;
//           };
//     var col =
//       args.col != undefined
//         ? args.col
//         : function(x) {
//             return "rgba(100,100,100," + (Math.random() * 0.3).toFixed(3) + ")";
//           };
//     var dis =
//       args.dis != undefined
//         ? args.dis
//         : function() {
//             if (Math.random() > 0.5) {
//               return (1 / 3) * Math.random();
//             } else {
//               return (1 * 2) / 3 + (1 / 3) * Math.random();
//             }
//           };
pub fn default_texture(points: Vec<Vec<(f64, f64)>>) -> Vec<Drawing> {

    return texture(points, 
        0.0, 0.0, 
        400, 
        1.5, 0.2, 
        0, 0, 
        default_noise, 
        colour, 
        dis);
}

fn default_noise(x: f64) -> f64{
    return x / 30.0;
}

fn default_dis() -> f64 {
    let mut rng = rand::thread_rng();
    if (rng.gen::<f64>() > 0.5) {
        return rng.gen() / 3.0;
    } else {
        return (2.0 + rng.gen) / 3.0;
    }

}

pub fn texture(
    points: Vec<Vec<(f64, f64)>>,
    x_offset:f64,
    y_offset:f64,
    tex: usize,
    width: f64,
    length: f64,
    sha: f64,
    ret: bool,
    noise: fn(f64) -> f64,
    colour: fn(f64) -> RGB,
    dis: fn() -> f64,
) -> Vec<Drawing> {

}
