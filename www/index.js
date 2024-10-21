import { Action } from "hunt-the-wumpus-wasm";
import { GameSession } from "hunt-the-wumpus-wasm";

const canvas = document.getElementById("hunting-grounds");

//const ctx = canvas.getContext("2d");

const game = GameSession.new(2, 2);

canvas.textContent = game.render();
