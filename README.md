Piston
======

A user friendly game engine written in Rust

[List of games made with Piston](https://github.com/PistonDevelopers/piston/issues/64)

[Piston online docs](http://pistondevelopers.github.io/docs/piston/piston)  

[How to contribute](https://github.com/PistonDevelopers/piston/issues/70)

| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/PistonDevelopers/rust-graphics) | [rust-graphics docs](http://pistondevelopers.github.io/docs/rust-graphics/graphics) |
| [rust-png](https://github.com/PistonDevelopers/rust-png) | [rust-png docs](http://pistondevelopers.github.io/docs/rust-png/png) |
| [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) | [rust-sdl2 docs](http://pistondevelopers.github.io/docs/rust-sdl2/sdl2/) |
| [glfw-rs](https://github.com/bjz/glfw-rs) | [glfw-rs docs](http://pistondevelopers.github.io/docs/glfw-rs/glfw/) |
| [gl-rs](https://github.com/bjz/gl-rs) | [gl-rs docs](http://pistondevelopers.github.io/docs/gl-rs/gl/) |
| [hgl-rs](https://github.com/cmr/hgl-rs) | [hgl-rs docs](http://pistondevelopers.github.io/docs/hgl-rs/hgl/) |

## Start new project with Piston

If you are starting a new project, [Rust-Empty](https://github.com/bvssvni/rust-empty) will automate the setup of directories for you.

When you have set up your project, you can build Piston.  
Piston can either be compiled as a static or dynamic library (or both).  
One way to simplify working with multiple projects on the same machine is to symlink the '.rlib' directly into the next project with the command `ln -s <from> <to>`.  

## How to build Piston

You can now use https://github.com/PistonDevelopers/piston-workspace repository to download and build Piston without having to set up symlinks manually.

Follow the instructions in the piston-workspace README before you add the libraries to your project.

The '.rlib' files are located in 'piston-workspace/piston/target/cpu-vendor-os/lib'.  

## Add libraries to your project

After building Piston, you need to add the '.rlib' files to your '/target/cpu-vendor-os/lib/' folder.  

The folder is created first time you build. If you do not see the folder, type `make exe`.  

## Goals

2D will be first priority, but we encourage people to experiment with 3D as well.  

* Test the design and performance of Rust-Graphics
* Experiment with Rust-ish game design and collect feedback
* Experiment with actor/events AI modelling
* Experiment with multiplayer architectures
* Make more people use Rust for game development
* Multi-platform game development
