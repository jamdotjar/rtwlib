# rtweekend.rs
Implementation of the raytracer from [raytracting in one weekend](https://github.com/RayTracing) in the crab language (rust).

This is mainly to learn about raytracing and rust Structs, Impl and Traits.
I also thought it would be fun to try and learn a little bit about the differences between rust and C++ in the translation process. 

I've converted this into a library, so it can be used in other projects. If you want to just mess around with the raytracer, you can use the main.rs file, which has some basic examples. If you dont want to mess with the source code, my project, [rtw.tui](https://github.com/jamdotjar/rtweekend-tui) lets you create and render scenes with a simple terminal interface.

The largest differnces from the origninal C++ raytracer are as follows:
Uses of the interval class have been replaced with rust's built in Range.

Images are currently exported in the ppm format, but I'm looking into PNGs, I want to implement encoding myself so this is a looong term goal.

## Future plans:

- Multithreading
- More object types
- BVH optimization
- PNG export support.

