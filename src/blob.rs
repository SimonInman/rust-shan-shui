use draw::Shape;
use noise::{NoiseFn, Perlin};
use rand::Rng;

// // a "blob" of somewhat solid colour
// //  Adapted from the following JS:
// function blob(x, y, args) {
//     var args = args != undefined ? args : {};
//     var len = args.len != undefined ? args.len : 20;
//     var wid = args.wid != undefined ? args.wid : 5;
//     var ang = args.ang != undefined ? args.ang : 0;
//     var col = args.col != undefined ? args.col : "rgba(200,200,200,0.9)";
//     var noi = args.noi != undefined ? args.noi : 0.5;
//     var ret = args.ret != undefined ? args.ret : 0;
//     var fun =
//       args.fun != undefined
//         ? args.fun
//         : function(x) {
//             return x <= 1
//               ? Math.pow(Math.sin(x * Math.PI), 0.5)
//               : -Math.pow(Math.sin((x + 1) * Math.PI), 0.5);
//           };

//     var reso = 20.0;
//     var lalist = [];
//     for (var i = 0; i < reso + 1; i++) {
//       var p = (i / reso) * 2;
//       var xo = len / 2 - Math.abs(p - 1) * len;
//       var yo = (fun(p) * wid) / 2;
//       var a = Math.atan2(yo, xo);
//       var l = Math.sqrt(xo * xo + yo * yo);
//       lalist.push([l, a]);
//     }
//     var nslist = [];
//     var n0 = Math.random() * 10;
//     for (var i = 0; i < reso + 1; i++) {
//       nslist.push(Noise.noise(i * 0.05, n0));
//     }

//     loopNoise(nslist);
//     var plist = [];
//     for (var i = 0; i < lalist.length; i++) {
//       var ns = nslist[i] * noi + (1 - noi);
//       var nx = x + Math.cos(lalist[i][1] + ang) * lalist[i][0] * ns;
//       var ny = y + Math.sin(lalist[i][1] + ang) * lalist[i][0] * ns;
//       plist.push([nx, ny]);
//     }

//     if (ret == 0) {
//       return poly(plist, { fil: col, str: col, wid: 0 });
//     } else {
//       return plist;
//     }
//   }

//         : function(x) {
//             return x <= 1
//               ? Math.pow(Math.sin(x * Math.PI), 0.5)
//               : -Math.pow(Math.sin((x + 1) * Math.PI), 0.5);
//           };
fn default_f(x: f64) -> f64 {
    let pi = std::f64::consts::PI;
    if x <= 1.0 {
        return (x * pi).sin().sqrt();
    } else {
        return -1.0 * ((x + 1.0) * pi).sin().sqrt();
    }
}

//     var len = args.len != undefined ? args.len : 20;
//     var wid = args.wid != undefined ? args.wid : 5;
//     var ang = args.ang != undefined ? args.ang : 0;
//     var col = args.col != undefined ? args.col : "rgba(200,200,200,0.9)";
//     var noi = args.noi != undefined ? args.noi : 0.5;
//     var ret = args.ret != undefined ? args.ret : 0;
pub(crate) fn default_blob(x: f64, y: f64) -> Shape {
    return blob(x, y, 40.0, 10.0, 0.0, 0.5, true, &default_f);
}

fn blob(
    x: f64,
    y: f64,
    length: f64,
    width: f64,
    ang: f64, // todo maybe a float?
    // colour: Color,
    noise: f64,
    ret: bool,
    f: &dyn Fn(f64) -> f64,
) -> Shape {
    if ang != 0.0 {
        todo!();
    }

    let resolution = 20;

    let lengths_and_angles = generate_lengths_and_angles(resolution, length, width, f);

    // let noise_list = naive_noise_list(lengths_and_angles.len());
    let noise_list = generate_noise_list(lengths_and_angles.len());
    // let looped_noise: Vec<f64> = loop_noise(noise_list);

    // todo LOOPNOISE
    //
    //
    let mut points_list = vec![];
    for i in 0..(lengths_and_angles.len()) {
        let this_noise = noise_list[i] * noise + (1.0 - noise);
        // let this_noise = looped_noise[i] * noise + (1.0 - noise);
        // let this_angle = lengths_and_angles[i].1 + ang;
        // let nx = x + this_angle.cos() * lengths_and_angles[i].0 * this_noise;
        // let ny = y + this_angle.cos() * lengths_and_angles[i].0 * this_noise;

        // points_list.push((nx, ny));

        let fraction: f64 = (i as f64) / resolution as f64;
        let p = fraction * 2.0;
        let xo = length / 2.0 - ((p - 1.0).abs() * length);
        let yo = (f(p) * width) / 2.0;
        let nx = x + xo * this_noise;
        let ny = y + yo * this_noise;
        points_list.push((nx, ny));
    }

    if ret {
        println!("returning a line_from with {} points", points_list.len());
        return super::line_from(points_list);
    } else {
        todo!()
    }
}

// function loopNoise(nslist) {
//     var dif = nslist[nslist.length - 1] - nslist[0];
//     var bds = [100, -100];
//     for (var i = 0; i < nslist.length; i++) {
//       nslist[i] += (dif * (nslist.length - 1 - i)) / (nslist.length - 1);
//       if (nslist[i] < bds[0]) bds[0] = nslist[i];
//       if (nslist[i] > bds[1]) bds[1] = nslist[i];
//     }
//     for (var i = 0; i < nslist.length; i++) {
//       nslist[i] = mapval(nslist[i], bds[0], bds[1], 0, 1);
//     }
//   }
// function mapval(value, istart, istop, ostart, ostop) {
//     return (
//       ostart + (ostop - ostart) * (((value - istart) * 1.0) / (istop - istart))
//     );
//   }

fn loop_noise(noise_list: Vec<f64>) -> Vec<f64> {
    // this is the diff once we loop our shape back around - we would like it to be small to 
    // get smooth shapes
    let dif = noise_list.last().unwrap() - noise_list.first().unwrap();
    // let upper_bound = 100.0;
    // let lower_bound = -100.0;

//       nslist[i] += (dif * (nslist.length - 1 - i)) / (nslist.length - 1);
    let length_minus_one: f64 = (noise_list.len() - 1) as f64;

    // this just linear interpolates from dif -> 0 as you go through.
    let delta_list: Vec<f64> = (0..noise_list.len()).into_iter().map(
        |i| ( dif * (length_minus_one - i as f64)) / length_minus_one
     ) .collect();

     // This is essentially turning the noise into a "flatish" walk, with diff reduced to zero. 
     let new_noise_list: Vec<f64> = noise_list.into_iter().zip(delta_list)
     .map(|(a, b)| a+b).collect();


     let upper_bound = new_noise_list.iter()
.fold(-100.0 as f64, |a, &b| a.max(b));
     let lower_bound = new_noise_list.iter().fold(100.0 as f64, |a, &b| a.min(b));

     let noise_range = upper_bound - lower_bound;

     let out = new_noise_list.into_iter()
     .map(|noise| (noise - lower_bound)/noise_range).collect();

     return out;
//       ((value - istart) * 1.0) / (istop - istart)


}


fn naive_noise_list(len: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    return (0..len).into_iter().map(|_| rng.gen()).collect();
}

fn generate_noise_list(len: usize) -> Vec<f64> {
    let perlin = Perlin::new();

    let mut rng = rand::thread_rng();
    let noise_base: f64 = rng.gen::<f64>() * 10.0;

    let noise_for_index = |i| perlin.get([(i as f64) * 0.05, noise_base]);
    let out = (0..len).into_iter().map(noise_for_index).collect();

    return out;
}



// Should do the following:
//
//     var reso = 20.0;
//     var lalist = [];
//     for (var i = 0; i < reso + 1; i++) {
//       var p = (i / reso) * 2;
//       var xo = len / 2 - Math.abs(p - 1) * len;
//       var yo = (fun(p) * wid) / 2;
//       var a = Math.atan2(yo, xo);
//       var l = Math.sqrt(xo * xo + yo * yo);
//       lalist.push([l, a]);
//     }
//
// This seems to be "la" meaning "length and angle". Atan gets the angle between the +ve x-axis
// and the line from (0,0) to (x, y)
//
// So what are these x0 and y0 we create?
// p is taken over <resolution> gradiations from 0 to 2.
fn generate_lengths_and_angles(
    resolution: i32,
    length: f64,
    width: f64,
    f: &dyn Fn(f64) -> f64,
) -> Vec<(f64, f64)> {
    let mut out = vec![];

    for i in 0..resolution + 1 {
        let fraction: f64 = (i as f64) / resolution as f64;
        let p = fraction * 2.0;
        let xo = length / 2.0 - ((p - 1.0).abs() * length);
        let yo = (f(p) * width) / 2.0;
        let a = yo.atan2(xo);
        let l = (xo * xo + yo * yo).sqrt();
        out.push((l, a));
    }
    return out;
}
