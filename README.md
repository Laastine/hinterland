# Hinterland

[![Build Status](https://travis-ci.org/Laastine/hinterland.svg?branch=master)](https://travis-ci.org/Laastine/hinterland)
[![Build status](https://ci.appveyor.com/api/projects/status/q30iw99u5f3ua237?svg=true)](https://ci.appveyor.com/project/Laastine/hinterland)

Isometric shooter game written in Rust.<br> Works on Linux, MacOS and Window.<br/>
Download from [Github releases](https://github.com/Laastine/hinterland/releases) page.

## Preview

<img src="assets/hinterland-gl-2018-09-21.gif">

## Project overview
- [Blog](https://laastine.kapsi.fi/code/2018/06/18/hinterland-status-update.html)
- [Project's task board](https://github.com/Laastine/hinterland/projects/1)

## Build & Run

`cargo run --release`

## Controls

`W,A,S,D` - Character move<br/>
`Ctrl + Mouse left` - Fire<br/>
`R` - Reload weapon (10 bullets per mag)<br/>
`Z` - zoom in<br/>
`X` - zoom out<br/>
`Esc` - exit

## Development

Run windowed mode with `cargo run --features "windowed godmode framerate"`

## External asset licence list

* Character: [graphics](http://opengameart.org/content/tmim-heroine-bleeds-game-art) Creative Commons V3
* Zombie [zombie](http://opengameart.org/content/zombie-sprites) Creative Commons V3
* Audio: [pistol](http://opengameart.org/content/chaingun-pistol-rifle-shotgun-shots) Creative Commons V3
* Map: [graphics](http://opengameart.org/content/tiled-terrains) GPL + Creative Commons V3

## Source code license

[Apache License 2.0](https://github.com/Laastine/hinterland/blob/master/LICENSE)
