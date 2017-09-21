# Zombie shooter

[![CircleCI](https://circleci.com/bb/laastine-ci/zombie-shooter/tree/master.svg?style=svg&circle-token=6f849c254dd78a3b0a19eccb75197d5325235b3f)](https://circleci.com/bb/laastine-ci/zombie-shooter/tree/master)

Simple isometric game written in Rust.<br/>
Project started as SDL2, but was later converted to use gfx-rs.

- Currently rebuilding features which were present in earlier [SDL2 version](http://laastine.kapsi.fi/kuvat/hackandslash.gif).

<img src="assets/zombie-shooter-gl.gif" alt="preview">

## Build

```bash
cargo install
cargo run
```

## Controls

`Arrow keys`<br/>
`+` - zoom in<br/>
`-` - zoom out<br/>
`Esc` - exit

## Development

Log frame render speed with
`cargo run --features dev`

Tested with Rust 1.20

## Asset licences

* Character: [graphics](http://opengameart.org/content/tmim-heroine-bleeds-game-art) Creative Commons V3
* Zombie [zombie](http://opengameart.org/content/zombie-sprites) Creative Commons V3
* Audio: [pistol](http://opengameart.org/content/chaingun-pistol-rifle-shotgun-shots) Creative Commons V3
* Map: [graphics](http://opengameart.org/content/tiled-terrains) GPL + Creative Commons V3
