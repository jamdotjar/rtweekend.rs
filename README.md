# rtweekend.rs
Implementation of the raytracer from [raytracting in one weekend](https://github.com/RayTracing) in the crab language (rust).

This is mainly to learn about raytracing and rust Structs, Impl and Traits.
I also thought it would be fun to try and learn a little bit about the differences between rust and C++ in the translation process. 

The largest differnces from the origninal C++ raytracer are as follows:
Uses of the interval class have been replaced with rust's built in Range.


Images are currently in the ppm format, but I'm looking into PNGs, I want to implement encoding myself so this is a looong term goal.

Todos:

- Multithreading
- PNG support

