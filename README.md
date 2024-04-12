# `cube.rs`

This is a simple reimplementation of Servet Gulnaroglu's ([@servetgulnaroglu](https://github.com/servetgulnaroglu)) `cube.c` program.

![A GIF of a spinning ASCII cube in the terminal](meta/cube_rs.gif)

You can see the repository with the original source code [here](https://github.com/servetgulnaroglu/cube.c), or watch the YouTube video where he coded it [here](https://youtu.be/p09i_hoFdd0).


## Running `cube.rs`

Despite the tongue-and-cheek name of the repository, the actual project runs from a `main.rs` file like any regular Rust project created from running `cargo new`. Running the code is as simple as downloading the repository, navigating to the project folder in a terminal, and running `cargo run`. You may need to resize your terminal to fit the animation. To quit the animation, just use `ctrl + c` to end the program.

I've included some sample animations in the repository, located in `samples.rs`. By default, the program runs the simple example, which features a single cube spinning in the center of the frame. You can edit `main.rs` to run one of the other sample animations, but just be aware that since each animation runs in an infinite loop, multiple function calls to sample animations inside the `main()` function will have no effect, as only the first one will run.

## From C to Rust

When originally porting this program to Rust, I wrote it as similarly to the original code as possible. However, there's a number of things that the original code does that simply aren't legal in Rust. For example, most of the variables in the original program are declared globally, allowing them to be reused every frame. In some cases this wasn't important, but several of the variables had to retain their values between frames for the code to work. As Rust doesn't allow such file-global variables (and `const` variables wouldn't work in this context), I had to wrap all the necessary variables in a `struct` and pass that `struct` between all the functions to be accessible.

Once I got the code working and had a clearer understanding of how it worked, I decided to refactor it to be more Rust-y. That decision turned out to be good practice for me, as it forced me to both evaluate how I thought the code should be reorganized in Rust, and to understand the code even better than I did before. I've detailed some of my thought-process below.

#### Structural Changes

Pun aside, I felt that moving most of the code into structs made the most sense, since Rust's object-oriented features offer some more organizational power than C does, and its rigid type system encourages you to use them. After some deliberation, I ended up organizing the code into `Cube`, `Point`, and `Screen` structs.

The `Cube` struct holds all the variables that are unique to each cube on-screen; namely, its size, positional offset, and the rotation of its three axis. It also implements the public `render_frame()` function, which is what actually draws that specific `Cube` to the buffers in the `Screen` struct; it's essentially a wrapper for the large, nested `for` loops found in the `main()` function of the original code. `render_frame()` calls the private `render_surface()` function, which is just a reimplementation of the `calculateForSurface()` function in the original code, adjusted for the use of `struct`s.

The `Screen` struct is what contains the actual screen buffers, as well as the size of the screen that gets printed to the console. To allow the screen size to be determined at runtime, the buffers are implemented as `Vec<>` instead of regular arrays, since regular arrays require a size that's knowable at compile time; the buffers never actually resized after they're instantiated. `Screen` also implements the public `print_frame()` function, which is what actually prints the frame to the terminal.

The `Point` struct exists entirely for performance reasons. The variables calculated in the `Cube`'s `render_surface()` function are derived entirely from the values in the `Cube`'s variables, and as such you could declare them anew every time the function runs. However, given that the function runs for every point on every face of the `Cube`, and declares seven variables every time, that seemed like an incredible waste of performance. As such, the `Point` struct acts as a wrapper for a number of public variables, and doesn't implement any functions beyond a constructor.

#### Functional Changes

Because of the aforementioned structural changes, adding and remove cubes in a given animation becomes quite straightforward, making the overall rendering loop much easier to understand. Simply instantiate all the `Cube` structs you want along side a `Screen` and `Point` struct, setting any size differences and positional offsets in the process. Then, in the rendering loop, simply call the `render_frame()` function on every cube before calling the `print_frame()` function of the `Screen` struct. Then rotate the cubes individually with their `rotate()` function, clear the screen buffers with `reset_buffers()`, and sleep the thread for as long as you want between frames.

Voilà! You have spinning cubes!


## License

I am unaware of any licenses on the original `cube.c` code. Therefore, since I firmly believe that choosing **any** license is better than having no license at all, I have chosen to follow much of the Rust community and license this work under the dual MIT / Apache-2.0 license. If it comes to my attention that the original work is licensed in some way, I'll reevaluate this section and update it as appropriate.

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.