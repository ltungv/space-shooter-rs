# SpaceShooterRS

Simple space shooter game written in [Rust](https://www.rust-lang.org) using [Bevy](https://bevyengine.org) game engine.

## Goals

This is me trying to design and implement a complete game using the Entity-Component-System paradigm. Before this project, I've tried out game development using other Rust game engines/ECS libraries, namely [Amethyst](https://amethyst.rs) (this uses [specs](https://github.com/amethyst/specs) as the ECS library) and [ggez](https://ggez.rs) together with [legion](https://github.com/amethyst/legion), but I found those quite complex and unintuitive for a beginner like me.

## Todos

Still thinking about the core mechanics and other interesting twists that can be added to the game.

- [x] player ship controls
- [x] player ship shoots laser automatically
- [x] enemies are randomly spawned
- [x] collision enemies-laser
- [ ] player ship health and damage
- [ ] enemies health and damage
- [ ] enemies fire lasers (towards player's ship)
- [ ] enemies moving patterns
- [ ] collision player-enemies
- [ ] collision player-laser
- [ ] collision laser-laser
- [ ] animations (for everything that can be animated)
- [ ] upgradables/consumables (damage multiplier, laser's patterns, rate of fire, movement speed, and etc.)
