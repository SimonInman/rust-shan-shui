

Day 4

Prev: 
 - Aims
 - Lines and blobs first



Today I'm trying to replicate the blob function <https://github.com/zverok/grok-shan-shui/blob/main/original.html#L578>, one of the fundamental drawing functions of the library. Once this is done, we should be able to start drawing some recognisable objects.

As the name tells us, this is basically shading a "blob", a solid block of colour. From reading this excellent post <https://zverok.github.io/advent2021/day02.html>, we determine that the blobs at the top of the trees are made by the blob function, so we have an idea of what the end result might look like. 

However, I found the code quite complicated. 

What I like to do in this situation is see if I can break up the code into independent pieces. Then I can understand the pieces individually, and represent them as chunks in my brain. Then hopefully the whole function can be described with not too many chunks for my brain to handle. 

First I'm going to write the function signature. The JS code has some optional arguments, but I don't think Rust has this feature, so I'll have to make them all explicit. 

```rust
fn blob(x: f32, y: f32, 
    len: int, 
    width: int,
    ang: int,  // todo maybe a float?
    colour: Color, 
    noise: f32, 
    ret: int,
    f: &dyn Fn(f32) -> f32,
) -> Shape {

}
```
(Later, when I'm calling blob, I'll think if there's some way to make it easier for the callers, such as having a default_blob() function that fills the default values for most of the parameters in. What that looks like precisely will depend on the main usages.)

The first thing I loook for is some variables that are only used over a small scope, normally to build another variable to use later. In this case, I saw that `lalist` is populated in a loop, and the other variables in the loop are not used outside that context. This made me think this was a good candidate for writing as a stand-alone function and understanding independently.

Here's the original code

```rust
     var reso = 20.0;
     var lalist = [];
     for (var i = 0; i < reso + 1; i++) {
       var p = (i / reso) * 2;
       var xo = len / 2 - Math.abs(p - 1) * len;
       var yo = (fun(p) * wid) / 2;
       var a = Math.atan2(yo, xo);
       var l = Math.sqrt(xo * xo + yo * yo);
       lalist.push([l, a]);
    }
```

And here's my fairly literal translation into Rust:
```rust
fn la_list(resolution: int, len: int, f: &dyn Fn(f32) -> f32) -> Vec<(f32, f32)> {
    let mut out = vec![];

    for i in 0..resolution+1 {
        let p = (i/resolution) * 2;
        let xo = len/2 - ((p-1).abs() * len);
        let yo = (f(p) * wid) /2;
        let a = y0.atan2(x0);
        let l = (xo*xo + yo*yo).sqrt();
        out.push((l, a));
    }
    return out;

}
```
Now what's going on here? 

I'm lucky in that I did a degree in Maths, so I no longer recoil with horror when I see trigonometry. 

When I look at definition of l, I notice that this is pythagoras' theorem for finding the length of the longest side of a right angled triangle. And looking up the docs of atan2, it's the angle between the (positive) x-axis and the point (xo, yo). If it's easier, here it is in diagram form:
```rust

          / (x0, y0)
         /
        /
   l   /
      /
     /
    /
   /
  /
 /
/ (a)
------------
```
l is the length of the line made out of slashes, and a is the angle between that line and the horizontal. 

So at this point, I'm thinking la_list stands for "line and angle list". But I'm still not clear what these are lines and angles _of_. 

We're generating xo and yo before getting their line/angle. Let's look more at the code and work out what's happening. 

```    rust
    for i in 0..resolution+1 {
        let p = (i/resolution) * 2;
```

So we're dividing the range (0, 2) into <resolution> segments and generating an (xo, yo) for each of these. 

```rust
        let xo = len/2 - ((p-1).abs() * len);
```

For xo, len is probably a length we can configure, and we can see that it exists in both terms of the above subtraction. We can do a small rearrangemnt to get: 

```rust
        let xo = len * (1/2 - (p-1).abs());
```

now if we remember that if p ranged from 0 to 2, then p-1 ranges from -1 to 1, (p-1).abs() ranges from 1 to zero and back to 1 again, and the whole xo term (ignoring length) ranges from -0.5 to +0.5 and back to -0.5.

<maybe demonstrate this with a table>

Recalling that X is the horizontal axis, this means that we are moving from the left hand side of the canvas to the right hand side, and then back. 


(If you're a maths-person, you probably recognise this as parametric equations for a curve.)

So now I think I have a good understanding of what's going on. We're going to be drawing a circle or circle-like shape (or an approximation to one, a polygon with many sides).

(Think about drawing a circle on a piece of paper, starting at the leftmost point - thinking only about the left-right motion you would move your hand from the left to the right and then back to the left.)

```        rust
let yo = (f(p) * wid) /2;
```

For `yo`, we have a function call. This is a bit of abstraction that allows the function to be more general, but at the cost of some readability.

In the same way that `x0` moves left and right as `p` moves from 0 to 2, `yo` will move as well - but in this case the movement is in the up-down axis. 

You can also see that we are multiplying the function call `f(p)` by the `wid` parameter. I had assumed this parameter stodd for "width", but we seem to be using it to change the up-down scale, which is a bit confusing to me.


The next part of blob concerns generating nslist. 

```rust
//     var nslist = [];
//     var n0 = Math.random() * 10;
//     for (var i = 0; i < reso + 1; i++) {
//       nslist.push(Noise.noise(i * 0.05, n0));
//     }
//     loopNoise(nslist);
```

As we call Noise.noise, I am thinking that nslist stands for "noise_list", a list of random numbers that I guess we will use to make our blobs look more random and therefore more arty. 

```rust
fn generate_noise_list(len: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    return (0..len).into_iter().map(|_| rng.gen()).collect();
}
```

The final non-returning chunk of the function is putting together plist

```        rust
//     var plist = [];
//     for (var i = 0; i < lalist.length; i++) {
//       var ns = nslist[i] * noi + (1 - noi);
//       var nx = x + Math.cos(lalist[i][1] + ang) * lalist[i][0] * ns;
//       var ny = y + Math.sin(lalist[i][1] + ang) * lalist[i][0] * ns;
//       plist.push([nx, ny]);
//     }
```        

Our plist is filled with things called (nx, ny), so I'm guessing that this stands for "points list", and when we look ahead to the main way this function returns, it calls poly(), which takes a list of points as arguments. 

```rust
//     if (ret == 0) {
//       return poly(plist, { fil: col, str: col, wid: 0 });
//     } else {
```

So what's happening in plist? We're looping over the lines and angles we created earlier. For each one, we're generating a pair of (x, y) co-ordinates, and our blob is a filled polygon that connects all these points.

```rust
//       var ns = nslist[i] * noi + (1 - noi);
```
This is a noise term, generated per point. It's also scaled by the function-level noi variant, which allows callers to control if all the points should be more or less noisy. 

```rust
//       var nx = x + Math.cos(lalist[i][1] + ang) * lalist[i][0] * ns;
```

lalist[i][1] is just the angle we worked out in lalist() function.

Warning
If you remember trigonometry, you might remember that the Cos functions allows us to get the length of the sides of triangles from angles. In this case. 

```Math.cos(lalist[i][1] + ang) * lalist[i][0] ```

tells us the length of the horizontal line in the below (which is xo), given the length l and angle a.

```

          / (x0, y0)
         /
        /
   l   /
      /
     /
    /
   /
  /
 /
/ (a)
------------

```

This seems strange to me - in lalist, we generated x0 and y0, and then used that to work out l and a. Now we take l and a, and work out x0. Why didn't we just hold onto x0?

[Insight from day 6: oh, we adjust the angle sometimes using ang. This is a way of rotating the initial shape, and will probably be needed at some point]

```rust
//       var ny = y + Math.sin(lalist[i][1] + ang) * lalist[i][0] * ns;
```

Tomorrow I'll actually write this code up and see if I can get it working.

DAY 5 

Got the code for the blobs working, but it's really irritating to have to specify the different numerical types of things. I guess when you're doing low-level stuff, this becomes an important consideration, but I don't care at all for this kind of thing. 

```rust
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

    let resolution = 60;

    let lengths_and_angles = generate_lengths_and_angles(resolution, length, width, f);

    // todo LOOPNOISE
    let noise_list = generate_noise_list(lengths_and_angles.len());
    let mut points_list = vec![];
    for i in 0..(lengths_and_angles.len()) {
        let this_noise = noise_list[i] * noise + (1.0 - noise);
        let this_angle = lengths_and_angles[i].1 + ang;
        let nx = x + this_angle.cos() * lengths_and_angles[i].0 * this_noise;
        let ny = y + this_angle.cos() * lengths_and_angles[i].0 * this_noise;

        points_list.push((nx, ny));
    }

    if ret {
        return super::line_from(points_list);
    } else {
        todo!()
    }
}

```

Unfortunately I got a blank canvas the first time I ran this, so I set about debugging it. I don't yet know how to run a debugger in Rust, so I chucked in some print statements after generating my lengths_and_angles and points list. 

I removed the "unneccesary" trigonometry, because I kept getting NaN numbers, and that seemed to be the fastest way to get the blobs working. [Actually, it turns out when I went over it that the trigonometry was needed to adjust the angle using the ang paramter. I've set myself up a branch to fail when I eventually hit that: ]

```rust
    if ang != 0.0 {
        todo!();
    }
```

I was still getting NaNs in my points list, but only a couple out of the 20, so I just added a terrible hack to discard any NaNs and log a warning.

Lo and behold, blobs!
![a blob](../tests/svg/spiky_blob1.svg "blob1")
![a blob](../tests/svg/spiky_blob2.svg "blob2")
![a blob](../tests/svg/spiky_blob3.svg "blob3")

They don't look quite the same though - I wondered if something was wrong (beyond the NaNs). Ah, I still have a TODO for "loop noise". THat's to do tomorrow then! [Edit from the future: Actually, I Implemented the noise_list wrong as well - nice work if you spotted this!]

DAY 6 

Today I was meant to investigate the loopNoise function. However I noticed an odd thing in the very first line: 

```rust
  function loopNoise(nslist) {
    var dif = nslist[nslist.length - 1] - nslist[0];
  ```

The first line of loopNoise calculates a diff between the first and last elements of the noise list. This confused me at first because I thought that the list was entirely random, and I didn't expect the first and last to have any special significance. It turns out that I read the code wrong yesterday and in fact the elements of it are distributed differently from one another:

```rust
   var n0 = Math.random() * 10;
    for (var i = 0; i < reso + 1; i++) {
      nslist.push(Noise.noise(i * 0.05, n0));
    }
```
Now, unfortunately for me, this Noise.noise is a custom implementaiton, and looks extremely hard for me to understand. It's 65+ lines of code with single letter variables being manipulated on every line (see here: https://github.com/zverok/grok-shan-shui/blob/main/original.html#L102 )

Luckily, there's some well named variables in the containing scope, and a link to some documentatino that suggests that this is an implementation of Perlin Noise. I have no idea what this is, but it's something I can google! I get told:

> Perlin noise is a popular procedural generation algorithm invented by Ken Perlin. It can be used to generate things like textures and terrain procedurally, meaning without them being manually made by an artist or designer. The algorithm can have 1 or more dimensions, which is basically the number of inputs it gets.

Nice! Textures and Terrain - sounds good given what we're trying to make. Another Google tells me that this already exists for rust, so all I need to do is use the Crate! <https://docs.rs/noise/latest/noise/>

It looks like I can just transliterate the JS code directly to get this

```rust
fn generate_noise_list(len: usize) -> Vec<f64> {
    let perlin = Perlin::new();

    let mut rng = rand::thread_rng();
    let noise_base: f64 = rng.gen::<f64>() * 10.0;

    let noise_for_index = |i| perlin.get([(i as f64) * 0.05, noise_base]);
    let out = (0..len).into_iter().map(noise_for_index).collect();

    return out;
}
```

I find the "into_iter()" and "collect()" calls a little irritating, but I'm sure there's a good reason for it. (I'm mildly tempted to write my own fmap function that will do this for me, but I doubt that's very Rust-y.)

I run the code again and they are less spiky now!

Before:
![a blob](../tests/svg/spiky_blob1.svg "blob1")
![a blob](../tests/svg/spiky_blob2.svg "blob2")
![a blob](../tests/svg/spiky_blob3.svg "blob3")

After:
![a blob](../tests/svg/less_spiky1.svg "blob1")
![a blob](../tests/svg/less_spiky2.svg "blob2")
![a blob](../tests/svg/less_spiky3.svg "blob3")

They're still a bit spiky on the left - will look into that tomorrow.


DAY 7 

Ok, after a day of diversion, we're back to looking at loopNoise! What's going on here then?

```rust
  function mapval(value, istart, istop, ostart, ostop) {
    return (
      ostart + (ostop - ostart) * (((value - istart) * 1.0) / (istop - istart))
    );
  }
  function loopNoise(nslist) {
    var dif = nslist[nslist.length - 1] - nslist[0];
    var bds = [100, -100];
    for (var i = 0; i < nslist.length; i++) {
      nslist[i] += (dif * (nslist.length - 1 - i)) / (nslist.length - 1);
      if (nslist[i] < bds[0]) bds[0] = nslist[i];
      if (nslist[i] > bds[1]) bds[1] = nslist[i];
    }
    for (var i = 0; i < nslist.length; i++) {
      nslist[i] = mapval(nslist[i], bds[0], bds[1], 0, 1);
    }
  }
  ```

This code modifies the list in place, but I will return a new list, which I assume is more Rust-y. 

It took quite a while to understand what was going on here, and I'm grateful to my friend Austin for jumping on a call with me  to talk it through. 

The crux lies in understanding 
1) these are noise terms that will be applied on a pointwise basis
2) The Perlin noise we developed yesterday is designed to be "smooth" in some sense. That is, unlike the first naive go at randomness, where the points were independent:

![a blob](../tests/svg/spiky_blob1.svg "blob1")

our efforts from yesterday are much smoother: 

![a blob](../tests/svg/less_spiky1.svg "blob1")

However, there is an exception on the left hand side of each of these pictures, where there's a much more jagged shape. This is because the Perlin noise we generated is designed to be smooth when used in a _linear_ fashion, but we are using it in a _circular_ fashion: when we apply the noises, we're applying it round the circumfrence of our circular-ish shape.

When I was thinking about this, I imagined the Perlin noise as generating the height of a landscape as we walk along it. It will be "smooth", rather than having cliffs. (N.B. We are sampling the Perlin noise at discrete points, but I don't think that changes the fundamental idea here.)

![Some perlin noise](Perlin1.png "That's some smoooooooth noise")

When we have our noise applied to a circular shape, we essentially want the value at the end to be similar to the value at the beginning, so that if we start going round again, there's not a large "jolt" at the start/end as we see in the pictures above. 

We can showcase this by "looping" the noise so that it starts again from the beginning:

![Some perlin noise](Perlin2.png "That's no longer smooth")

Now we can see there will is a big jagged cliff rather than a smooth curve, and that's the same effect we see on the left hand side of our blob here (the left hand side is both the start and the end of where we draw the blob)

![a blob](../tests/svg/less_spiky1.svg "blob1")

We're randomly moving the first and last points, and our goal is to have the random amount be similar, but because they are at opposite ends of the noise, there is no reason for the noise to be similar. 

Our goal in loop_noise is to make adjustments to the noise so that it still has randomness, but that the start and end values are similar. We want to eliminate the big red cliff from before. 

How will we do that?

The JS code starts by creating a difference termi (aka `delta_list`), here's the code transilterated into Rust:

```rust
fn loop_noise(noise_list: Vec<f64>) -> Vec<f64> {
    // this is the diff once we loop our shape back around - we would like it to be small to 
    // get smooth shapes
    let dif = noise_list.last().unwrap() - noise_list.first().unwrap();

    let length_minus_one: f64 = (noise_list.len() - 1) as f64;

    // this just linear interpolates from dif -> 0 as you go through.
    let delta_list: Vec<f64> = (0..noise_list.len()).into_iter().map(
        |i| ( dif * (length_minus_one - i as f64)) / length_minus_one
     ) .collect();
```

This difference term takes the height of our red cliff and makes a linear interpolation out of it. It's like drawing a straight line from the start of the line to the end of the line: 

![Some perlin noise with another line](Perlin3.png "That's no longer smooth")

What we're then going to do is consider the difference between the straight line and the curve, and have this as our noise - it's still random as we go along it, but at the start and end that difference is zero, so it will smoothly join up when we loop it.

(Note: Actually, `delta_list` is actually a mirror image of the line drawn, and we're going to add it rather than use the difference, but the principle is the same.)

```rust
     // This is essentially turning the noise into a "flatish" walk, with diff reduced to zero. 
     let new_noise_list: Vec<f64> = noise_list.into_iter().zip(delta_list)
     .map(|(a, b)| a+b).collect();

```

The rest of this function seems to just be finding the min and max of the resulting noise, and using that to do a normalisation.

```rust
     let upper_bound = new_noise_list.iter()
.fold(-100.0 as f64, |a, &b| a.max(b));
     let lower_bound = new_noise_list.iter().fold(100.0 as f64, |a, &b| a.min(b));

     let noise_range = upper_bound - lower_bound;

     let out = new_noise_list.into_iter()
     .map(|noise| (noise - lower_bound)/noise_range).collect();

     return out;
}
```

You might note that I haven't bothered with the mapval function. When trying to work out what it did, I found it simplified down to the penultimate line of the function above, so I didn't bother implementing it until it will be needed. 