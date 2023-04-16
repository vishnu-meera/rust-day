// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

// fn main_1() {
//     let width = 4;
//     let height = 7;
//     let depth = 10;
//     // 1. Try running this code with `cargo run` and take a look at the error.
//     //
//     // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
//     // doing `cargo run` should succeed and print something out.
//     {
//         let area = area_of(width, height);
//         println!("Area is {}", area);
//     }

//     // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
//     //    the code again and make sure it worked (you should get an area of 28).

//     // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
//     //    Create the `volume` function!  It should:
//     //    - Take three arguments of type i32
//     //    - Multiply the three arguments together
//     //    - Return the result (which should be 280 when you run the program).
//     //
//     // If you get stuck, remember that this is *very* similar to what `area_of` does.
//     //
//     println!("Volume is {}", volume(width, height, depth));
// }

// fn area_of(x: i32, y: i32) -> i32 {
//     // 2a. Fix this function to correctly compute the area of a rectangle given
//     // dimensions x and y by multiplying x and y and returning the result.
//     //
//     x*y
//     // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
//     //            `return` on the last line of a function. Change the last line to be a
//     //            "tail expression" that returns a value without using `return`.
//     //            Hint: `cargo clippy` will warn you about this exact thing.
// }

// fn volume(x: i32, y:i32, z:i32) -> i32 {
//     x*y*z
// }
use hello::print_difference;
use hello::print_array;
use hello::ding;
use hello::on_off;
use hello::print_distance;
use hello::encoding;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    print_difference(coords.0, coords.1);   // Uncomment and finish this line


    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr= [coords.0, coords.1] ;          // create an array literal out of parts of `coord` here
    print_array(coords_arr);        // and pass it in here (this line doesn't need to change)


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[2]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
    encoding();
}





