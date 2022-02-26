# Burger Time (NES) Patcher

## Usage
1. Copy Burger Time (NES) rom to the project root
2. Make sure the rom's filename is, `BurgerTime.nes`
3. Run the command `cargo run`
4. Output `PatchedBurgerTime.nes` starts with 80 lives, and 255 salt

## Known bugs
1. Grabbing a bonus item, which should increment your salt by 1, reduces your salt to 50

## FAQ
Q: Why 255 salt?
A: Salt is stored in an unsigned int, so the maximum is 255

Q: Why 80 lives?
A: I'm not sure yet. Anything more and the first death results in game over.

Q: Why do this at all?
A: Because this game is way too hard for me, and I wanted to see more levels :D

## Roadmap
1. Enemy slow-down
2. Reduce number of enemies
3. Unlimited salt
4. Fix bonus item bug