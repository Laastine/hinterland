# Zombie shooter

[![CircleCI](https://circleci.com/bb/laastine-ci/zombie-shooter/tree/master.svg?style=svg&circle-token=6f849c254dd78a3b0a19eccb75197d5325235b3f)](https://circleci.com/bb/laastine-ci/zombie-shooter/tree/master)

Simple isometric game written in Rust.<br/>
Project started as SDL2, but was later converted to use gfx-rs.

<img src="assets/zombie-shooter-gl.gif" alt="preview">

## Project overview
- [Project wiki](https://github.com/Laastine/zombie-shooter/projects/1)
- [Blog post](https://laastine.kapsi.fi/code/2018/01/07/zombie-shooter.html)

## Build

```bash
cargo install
cargo run
```

## Controls

`W,A,S,D` - Character move<br/>
`Mouse left` - Fire<br/>
`Z` - zoom in<br/>
`X` - zoom out<br/>
`Esc` - exit

## Development

Run windowed mode with `cargo run --features windowed`

Tested with Rust 1.23 (and nightly 1.25) with macOS and Linux

## Asset licences

* Character: [graphics](http://opengameart.org/content/tmim-heroine-bleeds-game-art) Creative Commons V3
* Zombie [zombie](http://opengameart.org/content/zombie-sprites) Creative Commons V3
* Audio: [pistol](http://opengameart.org/content/chaingun-pistol-rifle-shotgun-shots) Creative Commons V3
* Map: [graphics](http://opengameart.org/content/tiled-terrains) GPL + Creative Commons V3
