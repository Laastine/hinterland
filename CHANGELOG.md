# Release notes

All notable changes to this project will be documented in this file.

Note: 
  - OSX version works only when started from command line
  - Linux version needs libasound2-dev package installed.

## v0.3.11
  - Add command line argument parser

## v0.3.10
  - Bigger 128x128 map
  - Bullets are rendered by shooting direction instead of plain squares
  - Update code base for Rust 2018
  - Visualise maximum ammo clips

## v0.3.9
  - Ammunition sprite removed after picking after player picks it up

## v0.3.8
  - Pickable ammunition and weapon reloading logic

## v0.3.7
  - Limited ammo capacity visualized

## v0.3.6
  - Better zombie AI, zombies wander around and engage player only when close enough

## v0.3.5
  - Fix aiming bug, which occurred in some monitor configurations
  - Zombie attack only when player is close enough

## v0.3.4
  - Fix bullet flying direction bug

## v0.3.3
  - Character animation speed adjusted
  - Adjusted bullet size and collision logic
  - Replaced mpsc with crossbeam_channel crate

## v0.3.2
  - Fixed pathfinding bug

## v0.3.1
  - Render loop optimized
