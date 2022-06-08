# koss_selection_rustWebServer

### A rust based html server made for the task round of KOSS

This repo consists of 2 seperate versions of the task, one made with just the std libs, and one made with the warp crate which is based on hyper.rs

### Crates used
The warp crate is a framework for making webservers extremely simple to compose with the help of filters. It is built on top of hyper.rs, and is therefore very performant and well tested.

### Building 

Simply run `cargo build --release` to build the executable for both servers

Make sure to keep the web files and pictures in the same working directory as the executable in the std version, and keep the /webpages folder in the same working directory in the warp version
