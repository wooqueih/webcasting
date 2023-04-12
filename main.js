import init, { Wall, render } from "./webcasting_wasm/pkg/webcasting_wasm.js";

//startup
var canvas = document.getElementById("canvas");
canvas.height = canvas.style.height = window.innerHeight;
canvas.width = canvas.style.width = window.innerWidth;
var aspectRatio = canvas.width / canvas.height;
var ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;

/*for (let i = 0; i < imageData.data.length; i += 4) {
	imageData.data[i] = 10;
	imageData.data[i + 1] = 10;
	imageData.data[i + 2] = 15;
	imageData.data[i + 3] = 255;
}
ctx.putImageData(imageData, 0, 0);*/

var keyWDown = false;
var keyADown = false;
var keySDown = false;
var keyDDown = false;

var lastLoopTime;
var player = {
	x: 5.0,
	y: 5.0,
	angle: 0,
}
const DEGREES_IN_RADIANS = 0.0174533;
const FOV = 100.0;
const DOF = 20.0;

const MOVE_SPEED = 0.003;
const TURN_SPEED = 0.005;

const map = {
	height: 4,
	width: 5,
	data: [0, 1, 1, 1, 0,
		0, 1, 0, 0, 0,
		0, 0, 0, 1, 1,
		1, 1, 0, 0, 0,]
}

await init();

window.requestAnimationFrame(gameLoop);

function gameLoop(currentTime) {
	var deltaTime = currentTime - lastLoopTime;

	//input
	let adjusted_move_speed = MOVE_SPEED * deltaTime;
	let adjusted_turn_speed = TURN_SPEED * deltaTime;
	if (keyWDown) {
		player.x += Math.cos(player.angle) * adjusted_move_speed;
		player.y -= Math.sin(player.angle) * adjusted_move_speed;
	}
	if (keySDown) {
		player.x -= Math.cos(player.angle) * adjusted_move_speed;
		player.y += Math.sin(player.angle) * adjusted_move_speed;
	}
	if (keyADown) {
		player.angle += adjusted_turn_speed;
	}
	if (keyDDown) {
		player.angle -= adjusted_turn_speed;
	}
	let imageData = new ImageData(
		render(player.x, player.y, player.angle, canvas.width, canvas.height, map.data, map.width, map.height, DOF, FOV),
		canvas.width,
		canvas.height,
	);
	//console.log(data);
	ctx.putImageData(imageData, 0, 0);

	console.log(deltaTime, player);
	lastLoopTime = currentTime;
	window.requestAnimationFrame(gameLoop);
}
document.addEventListener("keydown", (e) => {
	switch (e.key) {
		case "w":
			keyWDown = true;
			break;
		case "a":
			keyADown = true;
			break;
		case "s":
			keySDown = true;
			break;
		case "d":
			keyDDown = true;
			break;
	}
});
document.addEventListener("keyup", (e) => {
	switch (e.key) {
		case "w":
			keyWDown = false;
			break;
		case "a":
			keyADown = false;
			break;
		case "s":
			keySDown = false;
			break;
		case "d":
			keyDDown = false;
			break;
	}
});
