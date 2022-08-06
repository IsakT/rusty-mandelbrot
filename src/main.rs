use num::complex::Complex;

fn calculate_mandelbrot(
    max_iterations: usize,
    real_min: f64,
    real_max: f64,
    imaginary_min: f64,
    imaginary_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    // init a vec that will hold vecs of all the rows
    let mut rows: Vec<_> = Vec::with_capacity(width);

    // loop through each y-axis coordinate.
    for pixel_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        // loop through each x-axis coordinate.
        // Now that we have both x and y coordinates, we have a point, or a pixel.
        for pixel_x in 0..width {
            // calculate pixel position as percentage of total width and height
            let pixel_x_percent = pixel_x as f64 / width as f64;
            let pixel_y_percent = pixel_y as f64 / height as f64;

            /*
            The complex plane is specified by the real_min, real_max, imaginary_min, imaginary_max.
            X-axis: real_min and real_max.
            Y-axis: imaginary_min and imaginary_max.
            Here we calculate the pixel position on the complex plane, just as you would
            on a regular 2D grid.
              Example:
                // on an x-axis where the min is 10 and the max is 30, halfway point is 20.
                x_axis_max    = 30
                x_axis_min    = 10
                x_axis_length = 20   // 30 - 10

                offset = 10          // always same as x_axis_min
                pixel_position = 0.5 // a pixel exactly half way (50%) on the x axis

                // to get the pixel position on the x axis:
                cx = pixel_position * x_axis_length + offset
                cx = 0.5 * 20 + 10 = 20
            */
            let x_axis_length = real_max - real_min;
            let offset = real_min;
            let cx = (pixel_x_percent * x_axis_length) + offset;

            // do the same as above, but for the y_axis.
            let y_axis_length = imaginary_max - imaginary_min;
            let offset = imaginary_min;
            let cy = (pixel_y_percent * y_axis_length) + offset;

            // c is the current point - the pixel coordinate - converted into a complex number.
            let c = Complex::new(cx, cy);

            // z is the starting point of the Mandelbrot, "in the middle" so to speak.
            let z = Complex { re: 0.0, im: 0.0 };

            // We now have what we need to calculate the Mandelbrot set equation:
            //   z * z + c
            let escaped_at = num_of_mandelbrot_iters_before_escape(c, z, max_iterations);

            // push the number of iterations the point took, into the row vec.
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

/*
Given a point in space (x, y), returns 'max_iterations' if point
belongs to the Mandelbrot set, else returns the number of iterations
before point escaped. (Escape value = 2.0)

Example:
  x = 0.40
  y = 0.91
  c_complex_number = (x+y)
  z_complex_number = (0+0)

  ////
  // FIRST ITERATION:
  // Mandelbrot equation: z * z + c
  // z_complex_number * z_complex_number + c_complex_number
  z = (0+0) * (0+0) + (0.40+0.91) = (0.40+0.91)

  ////
  // SECOND ITERATION:
  // c is still    (0.4+0.91)
  // z is now also (0.4+0.91)
  // formula for multiplying two complex numbers = (a+b) * (c+d) = ((ac-bd)+(ad+bc))

  (0.40 + 0.91) * (0.40 + 0.91) + (0.40 + 0.91) = ((0.4*0.4)-(0.92*0.92)) + ((0.4*0.92)+(0.92*0.4)) + (0.40 + 0.92)
                                                    (0.16 - 0.84 = -0.68) + (0.365 + 0.365 = 0.73)
                                (add 'c')             −0,68 + 0.40        +        0.73 + 0.92
                                (new value of 'z')        -0.28           +            1.65
                                (the resulting complex number)     (-0.28, i1.65)

  // new value of z after calculation
  z = (-0.28+1.65)

  ////
  // THIRD ITERATION:
  // c is still    (0.4+0.91)
  // z is now      (-0.28+1.65)

  // Next value of z
  (-0.28+1.65) * (-0.28+1.65) + (0.4+0.91) = (-2.24-0.00)

  // New value of z is now (-2.24 -0.00). Since the real number (x axis)
  // is more than 2.0 from the starting point (origo), it means that z will escape into infinity
  // if we keep iterating it, so it does not belong to the Mandelbrot set. When z escapes, we
  // stop the loop and return the number of iterations it took up until it exceeded the escape value.

  number_of_iterations_before_escape = 3

  // since the number of iterations were 3 in this particular case,
  // we return the number 3, which we display as '.' in the final image.

  return number_of_iterations_before_escape

*/
fn num_of_mandelbrot_iters_before_escape(
    c: Complex<f64>,
    mut z: Complex<f64>,
    max_iterations: usize,
) -> usize {
    // when z reaches radius of 2, it is going to speed off into infinity, so
    // we stop the iteration when it reaches this escape value.
    // If z never escapes, then z belongs to the Mandelbrot set and we display that pixel
    // as white-space in the final image.
    let escape_value = 2.0;

    for i in 0..=max_iterations {
        if z.re > escape_value        // z.re and z.im refers to its 'real' and 'imaginary' numbers
            || z.re < -escape_value
            || z.im > escape_value
            || z.im < -escape_value
        {
            // when or if z escapes, we count the number of iterations it has made up until that point.
            return i;
        }
        // the mathematical function for the Mandelbrot set.
        z = z * z + c;
    }

    // in case z never escapes, we just return the cap, which
    // in this case is the maximum number of iterations. Or else it will just continue forever.
    max_iterations
}

/*
Replaces each numeric mandelbrot-value in the grid with a char or whitespace.
Then prints each line to the display, row by row.
*/
fn render_mandelbrot(mandelbrot_points: Vec<Vec<usize>>) {
    for row in mandelbrot_points {
        let mut line = String::with_capacity(row.len());
        //                     ^^^^^^^^^^^^^
        // Since we know what length the final line-string will have, we can init the String with the with_capacity method.
        // This does not limit or restrain the String in any way,
        // it just optimizes the String to not reallocate each time something is appended to the String.
        // However, a new reallocation would take place if we would add more data beyond its initial buffer length.

        for pixel in row {
            let val = match pixel {
                // if max_iterations=1000 and num of escapes = 1000 (which means never escaped),
                // then the pixel was part of the Mandelbrot set.
                // Every other number of iterations are for displaying the "aura" surrounding the fractals.
                0..=2 => '¸',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '›',
                31..=100 => '-',
                101..=200 => '˛',
                201..=400 => '˙',
                401..=700 => '˛',
                701..=800 => '‘',
                801..=900 => '¨',
                901..=999 => '¸',
                1000 => ' ',
                _ => '!',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    // change width and height to suit your screen and terminal size.
    // Keep the ratio between width and height close to 3.50 for good results.
    // small screen: w: 100, h:28
    // full screen:  w: 230, h:66
    let screen_width = 230;
    let screen_height = 66;

    let max_iterations = 1000;
    let real_min = -2.0;
    let real_max = 1.0;
    let imaginary_min = -1.0;
    let imaginary_max = 1.0;
    let mandelbrot_points = calculate_mandelbrot(
        max_iterations,
        real_min,
        real_max,
        imaginary_min,
        imaginary_max,
        screen_width,
        screen_height,
    );

    render_mandelbrot(mandelbrot_points);
}
