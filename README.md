# Hunt The Wumpus (in webassembly)
### [try it out!](https://bradeneverson.github.io/wasm-wumpus/)
It's been a long time since you last encountered the Wumpus, and since then, it's moved to the world of Webassembly.

This repo contains a pretty simple version of the classic [hunt the wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) game, using a 5x5 grid to represent the cave you're exploring. The game logic and state is written in Rust, compiled to webassembly using [wasm-pack](https://github.com/rustwasm/wasm-pack) and run through an HTML frontend with some simpel JS business logic for pushing the state and updating the UI :)

![basic ui](https://github.com/user-attachments/assets/7cbc1374-9946-4532-b339-5cc659109196)

Stay sharp in the cave, as you may run into the following obstacles along the way:
* **Big Bats** who will pick you up and drop you off in a different room, potentially very dangerously so 🦇
* **Bottomless Pits** which will, shockingly, make you fall forever. Not great 🕳️
* The formidable **Wumpus**, a beast who's rage knows no bounds, don't walk into the same room as it or you will surely meet an untimely demise 👹

In order to survive the Wumpus, you must use your keen senses to locate the room it's found in and shoot an arrow at it. Be warned, as if you miss, the Wumpus will scurry towards a different room.
